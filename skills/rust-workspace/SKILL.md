---
name: rust-workspace
description: Create a new Rust workspace with Ivan's personal Cargo metadata, strict Clippy lints, RTK, cargo-nextest and Justfile commands. Use when asked to create a Rust workspace with Ivan's settings from any project; do not apply to unrelated edits or replace an existing workspace.
---

# Rust workspace with Ivan's settings

Use the self-contained [templates](templates/) and
[scripts/new_workspace.py](scripts/new_workspace.py). Resolve these paths
relative to this skill, never relative to the current project.

Create a new directory using:

```sh
python3 <skill-directory>/scripts/new_workspace.py <destination> \
  --repository <actual-repository-url> [--name <package-name>] \
  [--description <project-description>]
```

Infer destination and package name from the request; clarify missing repository
URL rather than inventing it. The destination must not exist. The script only
copies template files and substitutes placeholders in file paths/content.
It does not invoke Cargo, initialize Git, install dependencies or create commits.
No cargo-generate dependency is needed.

The generated CLAUDE.md only imports AGENTS.md, which stays the single source
of project instructions (shared with other agents). Read the generated
AGENTS.md and RTK.md before implementation. Preserve the
workspace metadata and lint levels from templates/Cargo.toml. Rust lint
groups have priority=-1; the 2024 compatibility group uses underscores.
Use resolver 3 (MSRV-aware) in new workspaces; do not change the resolver of an existing workspace unasked. Packages must inherit workspace
metadata and explicitly opt into `[lints] workspace = true`.

Applications belong in apps/, libraries in crates/, tools in tools/. Add member
globs only once packages exist. Keep business logic out of main.rs/lib.rs/mod.rs,
allowing minimal entry-point delegation. Do not copy Bevy or game-specific settings.

## Lints

`just lint` runs Clippy and then `just lint-arch`: the Dylint libraries `rust_layout` (flat crate
root, declaration-only `mod.rs`/`lib.rs`, `//!` module docs, file size, plain `pub`) and
`rust_testing` (rstest style) from https://github.com/pengwin/rust_dylints, pinned by git tag in
`[workspace.metadata.dylint]`. `bevy_layout` is deliberately not included; it belongs to the
`bevy-workspace` skill. The starter is laid out to pass them (`src/app/mod.rs` with `run.rs` and
`run_tests.rs`, not a lone `src/app.rs`); keep new code in that shape. Thresholds are in
`dylint.toml`. `cargo-dylint`/`dylint-link` >= 6 must be installed (do not install them silently:
tell the user); the first `just lint-arch` makes rustup install the nightly pinned in that
repository, and the workspace `rust-version` must not exceed it.

README must retain installation instructions for Rust, rustfmt, Clippy, just,
cargo-nextest, cargo-dylint with dylint-link (>= 6), RTK, Bash and the native linker/toolchain. Add the new application's
actual system/runtime dependencies. LICENSE is a clearly labeled proprietary
placeholder; adapt it if the user specifies a license.

Run `just fmt`, `just build`, `just lint`, `just test` and `just run` from the
new workspace; use `just test-doc` for libraries with doctests. Use Justfile
rather than direct Cargo build/test/lint commands. The starter includes an rstest bootstrap smoke test (`app/run_tests.rs`);
nextest allows other empty suites with a warning. Keep Cargo.lock available for
version control, but do not initialize Git unless separately requested.

Report the destination, run command and verification results, including any empty
test suite or missing dependencies. Do not silently install global tools.
