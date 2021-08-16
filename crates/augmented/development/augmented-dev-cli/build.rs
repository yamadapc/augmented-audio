use std::process::Command;

fn main() {
    {
        println!(
            "cargo:rustc-env=PROFILE={}",
            std::env::var("PROFILE").unwrap()
        );
    }
    {
        let output = Command::new("git")
            .args(&["rev-parse", "HEAD"])
            .output()
            .unwrap();
        let git_rev = String::from_utf8(output.stdout).unwrap().trim().to_string();
        println!("cargo:rustc-env=GIT_REV={}", git_rev);
    }
    {
        let output = Command::new("git")
            .args(&["rev-parse", "--short", "HEAD"])
            .output()
            .unwrap();
        let git_rev = String::from_utf8(output.stdout).unwrap().trim().to_string();
        println!("cargo:rustc-env=GIT_REV_SHORT={}", git_rev);
    }
}
