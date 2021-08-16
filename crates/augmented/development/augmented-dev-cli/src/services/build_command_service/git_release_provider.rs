use mockall::automock;

#[automock]
pub trait GitReleaseProvider {
    fn get_key(&self, pkg_version: &str) -> String;
}

#[derive(Default)]
pub struct GitReleaseProviderImpl {}

impl GitReleaseProvider for GitReleaseProviderImpl {
    fn get_key(&self, pkg_version: &str) -> String {
        let output = std::process::Command::new("git")
            .args(&["rev-parse", "--short", "HEAD"])
            .output()
            .expect("Failed to run git rev-parse");
        let git_rev = String::from_utf8(output.stdout)
            .expect("Failed to get git rev-parse output")
            .trim()
            .to_string();
        format!("release-{}-{}", pkg_version, git_rev)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_release_key() {
        let provider = GitReleaseProviderImpl::default();
        let key = provider.get_key("0.0.1");
        assert!(key.contains("0.0.1"));
        assert!(key.len() > "0.0.1".len());
    }
}
