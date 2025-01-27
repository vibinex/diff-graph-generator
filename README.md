To run application in a linux/unix environment - 
1. Install Rust
2. In the prompts directory, create a file hf_token with the huggingface token
3. Use `cargo run` with the following variables - 
```
GIT_DIR - path to the git repository on the local file system which needs to be processed
OLD_COMMIT_ID - the base commit id against which diff graph needs to be generated
NEW_COMMIT_ID - the head commit id against which diff graph needs to be generated
GIT_URL - remote git url of repo used for pulling and commiting
PROMPTS_PATH - path to the prompts directory (included in the diffgraph generator repo) on local file system
OUTPUT_PATH - path to local directory where output html will be written 
```