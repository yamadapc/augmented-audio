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
            .args(["rev-parse", "--short", "HEAD"])
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
