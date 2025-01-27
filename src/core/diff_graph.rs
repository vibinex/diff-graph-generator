use std::fs::File;
use std::io::Write;

use crate::graph::mermaid_elements::generate_mermaid_flowchart;
// use crate::utils::user::ProviderEnum;
// use crate::utils::review::Review;
// use crate::core::github;
use crate::utils::gitops::StatItem;

// pub async fn send_diff_graph(review: &Review, excluded_files: &Vec<StatItem>, small_files: &Vec<StatItem>, access_token: &str) {
// 	let comment = diff_graph_comment_text(excluded_files, small_files, review).await;
// 	// add comment for GitHub
// 	if review.provider().to_string() == ProviderEnum::Github.to_string() {
// 		log::info!("Inserting comment on repo {}...", review.repo_name());
// 		github::comment::add_comment(&comment, review, &access_token).await;
// 	}

// 	// TODO: add comment for Bitbucket
// }

// async fn diff_graph_comment_text(excluded_files: &Vec<StatItem>, small_files: &Vec<StatItem>, review: &Review) -> String {
//     let mut comment = "Diff Graph:\n\n".to_string();  
    
//     let all_diff_files: Vec<StatItem> = excluded_files
//         .iter()
//         .chain(small_files.iter())
//         .cloned()  // Clone the StatItem instances since `iter` returns references
//         .collect(); // Collect into a new vector
//     if let Some(mermaid_text) = mermaid_comment(&all_diff_files, review).await {
//         comment += mermaid_text.as_str();
//     }
//     comment += "\nTo modify DiffGraph settings, go to [your Vibinex settings page.](https://vibinex.com/settings)\n";
//     return comment;
// }

// async fn mermaid_comment(diff_files: &Vec<StatItem>, review: &Review) -> Option<String> {
//     let flowchart_str_opt = generate_mermaid_flowchart(diff_files, review).await;
//     if flowchart_str_opt.is_none() {
//         log::error!("[mermaid_comment] Unable to generate flowchart for review: {}", review.id());
//         return None;
//     }
//     let flowchart_str = flowchart_str_opt.expect("Empty flowchart_str_opt");
//     let mermaid_comment = format!(
//         "### Call Stack Diff\n```mermaid\n{}\n```",
//         flowchart_str,
//     );
//     return Some(mermaid_comment);
// }

pub async fn generate_mermaid_html(diff_files: &Vec<StatItem>, prev_commit: &str, next_commit: &str, git_dir: &str, git_url: &str, output_path: &str) {
    mermaid_html(diff_files, prev_commit, next_commit, git_dir, git_url, output_path).await;
}

async fn mermaid_html(diff_files: &Vec<StatItem>, prev_commit: &str, next_commit: &str, git_dir: &str, git_url: &str, output_path: &str) {
    let flowchart_str_opt = generate_mermaid_flowchart(diff_files, prev_commit, next_commit, git_dir, git_url).await;
    if flowchart_str_opt.is_none() {
        log::error!("[mermaid_html] Unable to generate flowchart for commits: {}, {}", prev_commit, next_commit);
        return;
    }
    let flowchart_str = flowchart_str_opt.expect("Empty flowchart_str_opt");
    let html_template = format!(
        r#"
        <html>
          <body>
            Here is one mermaid diagram:
            <pre class="mermaid">
{flowchart_str}
            </pre>
            <script type="module">
              import mermaid from 'https://cdn.jsdelivr.net/npm/mermaid@11/dist/mermaid.esm.min.mjs';
              mermaid.initialize({{ startOnLoad: true }});
            </script>
          </body>
        </html>
        "#,
        flowchart_str = flowchart_str.trim()
    );

    // Specify the file path where the HTML will be written
    let file_path = format!("{}/diffgraph.html", output_path);

    // Create and write to the file
    let file_res = File::create(file_path);
    if file_res.is_err() {
        log::error!("[mermaid_html] Unable to create diff graph file: {:?}",
            file_res.expect_err("Empty error"));
        return;
    }
    let mut file_obj = file_res.expect("Uncaught file write error");
    let write_res = file_obj.write_all(html_template.as_bytes());
    if write_res.is_err() {
        log::error!("[mermaid_html] Unable to write diff graph to file system: {:?}",
        write_res.expect_err("Empty error"));
        return;
    }
    log::info!("Diff Graph HTML generated!!");
}

