# macos-bundle-resources
Wrapper on top of `CFBundleCopyResourceURL` to get resource URLs within a plug-in.

```rust
fn main() {
    let path = macos_bundle_resources::get_path(
        "com.beijaflor.TasV2", // <- Bundle ID
        "frontend/index.html", // <- Resource name
        None,                  // <- Resource extension
        None                   // <- Resource subdir
    );
    println!("{}", path); // => Will output the resource path on the FS
}
```