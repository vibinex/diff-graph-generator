use core::diff_graph::generate_mermaid_html;
use std::env;
use tokio::task;

mod core;
mod graph;
mod logger;
mod utils;

use crate::utils::gitops::get_diff_files;
use crate::utils::prompts::set_prompt_path;

#[tokio::main]
async fn main() {
	let git_dir = //"/home/tapishr/dev-profiler/pubsub-sa.json".to_owned();
	env::var("GIT_DIR").expect("GIT_DIR must be set");
	let old_commit_id = //"rtapish-fromserver".to_owned();
	env::var("OLD_COMMIT_ID").expect("OLD_COMMIT_ID must be set");
	let new_commit_id = //"rtapish-fromserver".to_owned();
	env::var("NEW_COMMIT_ID").expect("NEW_COMMIT_ID must be set");
	let git_url = //"rtapish-fromserver".to_owned();
	env::var("GIT_URL").expect("GIT_URL must be set");
	let prompts_path = //"rtapish-fromserver".to_owned();
	env::var("PROMPTS_PATH").expect("PROMPTS_PATH must be set");
	let output_path = //"rtapish-fromserver".to_owned();
	env::var("OUTPUT_PATH").expect("OUTPUT_PATH must be set");
	let logs_init_status = logger::init::init_logger();
	if !logs_init_status {
		log::warn!("[main] Unable to create file logger");
	}
	log::info!("Generating diff graph git repo in {} between {} and {}", &git_dir, &old_commit_id, &new_commit_id );
	set_prompt_path(prompts_path);
	let diff_files_opt = get_diff_files(&old_commit_id, &new_commit_id, &git_dir);
	if diff_files_opt.is_none() {
		log::error!("[main] Unable to get source files in diff");
	}
	let diff_files = diff_files_opt.expect("Empty diff_files_opt");
	generate_mermaid_html(&diff_files, &old_commit_id, &new_commit_id, &git_dir, &git_url, &output_path).await;
}