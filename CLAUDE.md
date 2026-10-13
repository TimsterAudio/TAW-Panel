# TAW Panel: working conventions

The shared panel crate for Timster's Audio Works' native apps (Library, Player, Equaliser, Analyser). It follows Drone Box's panel language; Drone Box's and the Equaliser's `docs/DESIGN.md` describe it.

## Hard rules

- **Look only.** No app logic, audio, file or network code here. Anything an app needs only for itself stays in the app.
- **Apps pin a commit.** Each version is a commit recorded in CHANGELOG.md; never rewrite history on a branch apps pin into. A change that alters how something looks or what a function takes is a new minor version (0.x.0); fixes are a patch (0.x.y).
- **Don't break callers casually.** Prefer adding a function or a field with a default over changing an existing signature. When a signature must change, note it in CHANGELOG.md under the new version with what callers do.
- **No network at runtime.** The fonts are built into the binary.

## Checks before pushing

```sh
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo clippy --target x86_64-pc-windows-gnu -- -D warnings
```

For anything visible, run `cargo run --example gallery` (under Xvfb in a cloud session) and look at both panels.

## Releasing a version

1. Move Unreleased in CHANGELOG.md into `## X.Y.Z – YYYY-MM-DD` and bump `version` in Cargo.toml.
2. Commit and push, then record the commit hash next to the version in CHANGELOG.md (cloud sessions can't push git tags; the user can add a `vX.Y.Z` tag on GitHub if wanted).
3. In each app that should have it, change its `taw-panel` `rev`, build, and look at it.
