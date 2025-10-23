use serde::Deserialize;
use serde::Serialize;
use std::process::Command;
use std::str;

#[derive(Debug, Serialize, Default, Deserialize, Clone)]
pub struct StatItem {
    pub filepath: String,
    additions: i32,
    deletions: i32,
}

pub fn commit_exists(commit: &str, directory: &str) -> bool {
    let output_res = Command::new("git")
        .arg("rev-list")
        .arg(commit)
        .current_dir(directory)
        .output();
    if output_res.is_err() {
        let e = output_res.expect_err("No error in output_res");
        log::debug!("[commit_exists] Failed to start git rev-list: {:?}", e);
        return false;
    }
    let output = output_res.expect("Uncaught error in output_res");
    if !output.status.success() {
        log::debug!(
            "[commit_exists] git rev-list, exit code: {:?}",
            output.status.code()
        );
        // for debugging
        match str::from_utf8(&output.stderr) {
            Ok(v) => log::error!("[commit_exists] git rev-list stderr = {:?}", v),
            Err(e) => {
                /* error handling */
                log::error!("[commit_exists] git rev-list stderr error {}", e)
            }
        };
        return false;
    }
    log::debug!(
        "[commit_exists] Execute git rev-list, exit code: {:?}",
        output.status.code()
    );
    if output.status.code() == Some(128) {
        // for debugging
        match str::from_utf8(&output.stderr) {
            Ok(v) => log::error!("[commit_exists] git rev-list stderr = {:?}", v),
            Err(e) => {
                /* error handling */
                log::error!("[commit_exists] git rev-list stderr error {}", e)
            }
        };
        return false;
    }
    // for debugging
    match str::from_utf8(&output.stderr) {
        Ok(v) => log::debug!("[commit_exists] git rev-list stderr = {:?}", v),
        Err(e) => {
            /* error handling */
            log::error!("[commit_exists] git rev-list stderr error {}", e)
        }
    };
    match str::from_utf8(&output.stdout) {
        Ok(v) => log::debug!("[commit_exists] git rev-list stdout = {:?}", v),
        Err(e) => {
            /* error handling */
            log::error!("[commit_exists] git rev-list stdout error {}", e)
        }
    };
    return true;
}

// pub async fn git_pull(review: &Review, access_token: &str) {
// 	let directory = review.clone_dir();
// 	log::debug!("[git_pull] directory = {}", &directory);
//     set_git_url(review.clone_url(), directory, &access_token, review.provider());
// 	let output_res = Command::new("git")
// 		.arg("pull")
// 		.current_dir(directory)
// 		.output();
// 	if output_res.is_err() {
// 		let e = output_res.expect_err("No error in output_res");
// 		log::error!("[git_pull] failed to execute git pull: {:?}", e);
// 		return;
// 	}
// 	let output = output_res.expect("Uncaught error in output_res");
// 	match str::from_utf8(&output.stderr) {
// 		Ok(v) => log::debug!("[git_pull] git pull stderr = {:?}", v),
// 		Err(e) => {/* error handling */ log::error!("[git_pull] git pull stderr error {}", e)},
// 	};
// 	match str::from_utf8(&output.stdout) {
// 		Ok(v) => log::debug!("[git_pull] git pull stdout = {:?}", v),
// 		Err(e) => {/* error handling */ log::error!("[git_pull] git pull stdout error {}", e)},
// 	};
// }

pub fn git_checkout_commit(directory: &str, commit_id: &str) {
    let output_res = Command::new("git")
        .arg("checkout")
        .arg(commit_id)
        .current_dir(directory)
        .output();
    if output_res.is_err() {
        let e = output_res.expect_err("No error in output_res");
        log::error!(
            "[git_checkout_commit] failed to execute git checkout: {:?}",
            e
        );
        return;
    }
    let output = output_res.expect("Uncaught error in output_res");
    match str::from_utf8(&output.stderr) {
        Ok(v) => log::debug!("[git_checkout_commit] git checkout stderr = {:?}", v),
        Err(e) => {
            /* error handling */
            log::error!("[git_checkout_commit] git checkout stderr error {}", e)
        }
    };
    match str::from_utf8(&output.stdout) {
        Ok(v) => log::debug!("[git_checkout_commit] git checkout stdout = {:?}", v),
        Err(e) => {
            /* error handling */
            log::error!("[git_checkout_commit] git checkout stdout error {}", e)
        }
    };
}

pub fn get_diff_files(
    prev_commit: &str,
    next_commit: &str,
    git_dir: &str,
) -> Option<Vec<StatItem>> {
    println!(
        "[get_diff_files] prev_commit = {}, next commit = {}, git_dir = {}",
        prev_commit, next_commit, git_dir
    );
    log::debug!(
        "[get_diff_files] prev_commit = {}, next commit = {}, git_dir = {}",
        prev_commit,
        next_commit,
        git_dir
    );
    let commit_range = format!("{}...{}", prev_commit, next_commit);
    let git_res = Command::new("git")
        .args(&["diff", &commit_range, "--numstat"])
        .current_dir(git_dir)
        .output();
    if git_res.is_err() {
        let commanderr = git_res.expect_err("No error in git command");
        log::error!(
            "[get_diff_files] git diff stat command failed to start : {:?}",
            commanderr
        );
        println!(
            "[get_diff_files] git diff stat command failed to start : {:?}",
            commanderr
        );
        return None;
    }
    let resultstat = git_res.expect("Uncaught error in git_res");
    let stat = resultstat.stdout;
    // parse the output
    let stat_res = str::from_utf8(&stat);
    if stat_res.is_err() {
        let staterr = stat_res.expect_err("No error in git command");
        log::error!(
            "[get_diff_files] git diff stat command parse error : {:?}",
            staterr
        );
        return None;
    }
    let statstr = stat_res.expect("Uncaught error in stat_res");
    log::debug!("[get_diff_files] statstr = {}", statstr);
    println!("[get_diff_files] statstr = {}", statstr);
    return Some(process_statitems(statstr));
}

// fn process_statoutput(statstr: &str) -> Option<(Vec<StatItem>, Vec<StatItem>)>{
//     let statvec = process_statitems(statstr);
//     let mut excluded_files = Vec::<StatItem>::new();
//     let mut filtered_files = Vec::<StatItem>::new();
//     let line_threshold = 5500;
//     for item in statvec {
//         // logic for exclusion
//         if (item.additions > line_threshold) ||
//         (item.deletions > line_threshold) ||
//         (item.additions + item.deletions > line_threshold) ||
// 		(item.deletions < 1) {
//             excluded_files.push(item);
//         }
//         else {
//             filtered_files.push(item);
//         }
//     }
//     return Some((excluded_files, filtered_files));
// }

fn generate_statitem(statitems: &Vec<&str>) -> StatItem {
    let statitem = StatItem {
        filepath: statitems[2].to_string(),
        additions: match statitems[0].to_string().parse() {
            Ok(adds) => adds,
            Err(e) => {
                log::error!("[generate_statitem] Unable to parse additions: {:?}", e);
                0 // default value
            }
        },
        deletions: match statitems[1].to_string().parse() {
            Ok(dels) => dels,
            Err(e) => {
                log::error!("[generate_statitem] Unable to parse deletions: {:?}", e);
                0 // default value
            }
        },
    };
    return statitem;
}

fn process_statitem(line: &str) -> Option<StatItem> {
    let statitems: Vec<&str> = line.split("\t").collect();
    if statitems.len() >= 3 {
        let statitem = generate_statitem(&statitems);
        return Some(statitem);
    }
    return None;
}

fn process_statitems(statstr: &str) -> Vec<StatItem> {
    let statlines = statstr.split("\n");
    let mut statvec = Vec::<StatItem>::new();
    for line in statlines {
        let statitem_opt = process_statitem(line);
        if statitem_opt.is_none() {
            continue;
        }
        let statitem = statitem_opt.expect("statitem is empty");
        statvec.push(statitem);
    }
    return statvec;
}
