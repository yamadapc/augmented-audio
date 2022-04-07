// Augmented Audio: Audio libraries and applications
// Copyright (c) 2022 Pedro Tacla Yamada
//
// The MIT License (MIT)
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.
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
        ">> Found project root at {} it'll be used as the CWD",
        root.to_str().unwrap()
    );
    std::env::set_current_dir(root).expect("Failed to cd onto the project root");
    println!(">> Listing submodules in submodules.json");

    let submodules = list_submodules();
    for submodule in submodules {
        validate_submodule(&submodule);
    }
}

fn validate_submodule(submodule: &SubmoduleDeclaration) {
    println!(">> Validating {} is up-to-date", submodule.path);
    let submodule_path = &submodule.path;
    let upstream_url = &submodule.upstream_url;
    let submodule_branch = submodule
        .branch
        .clone()
        .unwrap_or_else(|| String::from("master"));
    let _ = cmd_lib::run_cmd!(
        cd ${submodule_path}; git push
    );
    let _ = cmd_lib::run_cmd!(
        cd ${submodule_path}; git remote add upstream ${upstream_url}
    );
    let _ = cmd_lib::run_cmd!(
        cd ${submodule_path}; git fetch upstream
    );
    let _ = cmd_lib::run_cmd!(
        cd ${submodule_path}; git --no-pager log upstream/${submodule_branch} --format=oneline ^origin/${submodule_branch}
    );
    let _ = cmd_lib::run_cmd!(
        cd ${submodule_path}; git merge upstream/${submodule_branch}; git push
    );
}
