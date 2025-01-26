
use crate::{graph::{elements::MermaidGraphElements, graph_edges::graph_edges, graph_info::generate_diff_graph}, utils::gitops::{git_checkout_commit, StatItem}};

use super::utils::all_code_files;


pub async fn generate_mermaid_flowchart(diff_files: &Vec<StatItem>, prev_commit: &str, next_commit: &str, git_dir: &str, git_url: &str) -> Option<String> {
    let flowchart_content_res = generate_flowchart_elements(diff_files, prev_commit, next_commit, git_dir, git_url).await;
    if flowchart_content_res.is_none() {
        log::error!("[generate_mermaid_flowchart] Unable to generate flowchart content, commits: {}, {}", prev_commit, next_commit);
        return None;
    }
    let flowchart_content = flowchart_content_res.expect("Empty flowchart_content_res");
    let flowchart_str = format!(
        "%%{{init: {{ \
            'theme': 'neutral', \
            'themeVariables': {{ \
                'fontSize': '20px' \
            }}, \
            'flowchart': {{ \
                'nodeSpacing': 100, \
                'rankSpacing': 100 \
            }} \
        }} }}%%\n \
        \tflowchart LR\n{}",
        &flowchart_content
    );
    return Some(flowchart_str);
}

async fn generate_flowchart_elements(diff_files: &Vec<StatItem>, prev_commit: &str, next_commit: &str, git_dir: &str, git_url: &str) -> Option<String> {
    // generate full graph for base commit id
    git_checkout_commit(git_dir, prev_commit);
    let base_filepaths_opt = all_code_files(git_dir, diff_files);
    if base_filepaths_opt.is_none() {
        log::error!(
            "[generate_flowchart_elements] Unable to get file paths: {}", git_dir);
        return None;
    }
    let base_filepaths = base_filepaths_opt.expect("Empty base_filepaths_opt");
    // let base_commit_import_info = get_test_import_info();
    let diff_graph_opt = generate_diff_graph(prev_commit, next_commit, git_dir).await;
    log::debug!("[generate_flowchart_elements] diff_graph_opt = {:#?}", &diff_graph_opt);
    if diff_graph_opt.is_none() {
        log::error!(
            "[generate_flowchart_elements] Unable to generate diff graph for commit ids: {}, {}",
            prev_commit, next_commit);
        return None;
    }
    let diff_graph = diff_graph_opt.expect("Empty diff_graph_opt");
    let mut graph_elems = MermaidGraphElements::new();
    graph_edges(&base_filepaths, prev_commit, next_commit, git_dir, &diff_graph, &mut graph_elems).await;
    let elems_str = graph_elems.render_elements(prev_commit, next_commit, git_url);
    return Some(elems_str);
}