use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Look up from the `target` to find the project root directory based on it having a `.git` child.
fn project_root(target: &Path) -> Option<PathBuf> {
    let directory = std::fs::read_dir(target).unwrap();

    for file in directory {
        let file = file.unwrap();
        if file.file_name() == ".git" {
            return Some(file.path().with_file_name(""));
        }
    }
    if let Some(parent) = target.parent() {
        return project_root(parent);
    }

    None
}

#[derive(Serialize, Deserialize)]
struct SubmoduleDeclaration {
    path: String,
    url: String,
    upstream_url: String,
    branch: Option<String>,
}

fn list_submodules() -> Vec<SubmoduleDeclaration> {
    let submodules_file = std::fs::read_to_string(format!(
        "{}/submodules.json",
        project_root(&std::env::current_dir().unwrap())
            .unwrap()
            .to_str()
            .unwrap()
    ))
    .expect("Failed to read submodules.json");
    serde_json::from_str(&submodules_file).expect("Failed to parse submodules.json")
}

fn main() {
    let root = project_root(&std::env::current_dir().unwrap()).unwrap();
    println!(
        "Found project root at {} it'll be used as the CWD",
        root.to_str().unwrap()
    );
    std::env::set_current_dir(root);
    println!("Listing submodules in submodules.json");

    let submodules = list_submodules();
    for submodule in submodules {
        validate_submodule(&submodule);
    }
}

fn validate_submodule(submodule: &SubmoduleDeclaration) {
    println!("Validating {} is up-to-date", submodule.path);
    let submodule_path = &submodule.path;
    let upstream_url = &submodule.upstream_url;
    let submodule_branch = submodule
        .branch
        .clone()
        .unwrap_or_else(|| String::from("master"));
    cmd_lib::run_cmd!(
        cd ${submodule_path}; git remote add upstream ${upstream_url}
    );
    cmd_lib::run_cmd!(
        cd ${submodule_path}; git fetch upstream
    );
    cmd_lib::run_cmd!(
        cd ${submodule_path}; git --no-pager log upstream/${submodule_branch} --format=oneline ^origin/${submodule_branch}
    );
    cmd_lib::run_cmd!(
        cd ${submodule_path}; git merge upstream/${submodule_branch}
    );
}
