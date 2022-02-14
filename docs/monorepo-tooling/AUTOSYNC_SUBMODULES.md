# Monorepo & Submodules
This is a mono-repository and several dependencies are vendored as forked submodules. A script can sync the upstreams:
```shell
cargo run --package autosync-submodules
```

This will fetch all upstreams, list the new commits & try to merge them.
