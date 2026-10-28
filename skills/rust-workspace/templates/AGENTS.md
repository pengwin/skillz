# Rust workspace

## Mandatory instruction compliance

Before editing code:

1. Read AGENTS.md and every referenced instruction file completely.
2. Create an internal checklist of applicable requirements.
3. Do not replace an explicitly requested architecture or existing implementation
   with a simplified alternative without user approval.
4. Treat architecture requirements as acceptance criteria alongside build and lint checks.

## Workspace structure

- Put applications in apps/, libraries in crates/, development tools in tools/.
- Add a workspace member glob only when that directory contains a package.
- Keep shared package metadata, dependencies and lints in root Cargo.toml.
- Every member inherits workspace package metadata and explicitly enables
  `[lints] workspace = true`.
- Inherit shared dependencies with `dependency.workspace = true`.
- Preserve configured lint levels and priorities. Do not add broad allow
  attributes to make checks pass; explain necessary narrow exceptions.
- Keep Cargo.lock available for version control. Do not initialize Git unless requested.
- Add custom profiles, rustflags, pinned toolchains or nextest settings only when needed.

## Rust code design

- Keep main.rs, lib.rs and mod.rs limited to declarations, module wiring and
  intended re-exports. Allow minimum entry-point delegation in main.rs.
- Put business logic in separate implementation files.
- Keep implementation details private. Use plain `pub` or private: `pub(crate)`,
  `pub(super)` and `pub(in ...)` are rejected by `layout_restricted_visibility`.
- Layout is enforced by the Dylint library `rust_layout` (`just lint-arch`): the crate root
  `src/` holds only `lib.rs`/`main.rs` and module directories (a module is `foo/mod.rs`, never a
  lone `src/foo.rs`; `layout_flat_crate_root`); `lib.rs`, `mod.rs` and `main.rs` only declare
  modules and re-export (`layout_root_file_logic`); every `mod.rs` and `lib.rs` starts with a `//!`
  comment of at least three words saying what the module owns (`layout_missing_module_doc`); a file
  has at most 500 lines (`layout_oversized_file`; generated files are skipped through
  `ignored_paths`); names in `forbidden_file_names` of `dylint.toml` are not allowed. Thresholds
  and lists live in `dylint.toml`; never silence a `layout_*` lint with a crate-wide allow, fix the
  structure (a narrow `#[cfg_attr(dylint_lib = "rust_layout", allow(layout_...))]` on one item is
  the exception to explain).
- Tests use rstest (`rust_testing`): `#[rstest]`, never a plain `#[test]` (`testing_plain_test`);
  shared setup is a `#[fixture]` taken as an argument, not a helper function or a `let` that builds
  test data in the body (`testing_helper_fn`, `testing_inline_arrange`); variants of one test are
  `#[case]` arguments. Put tests in `*_tests.rs` beside the code.
- Add tests for application behavior. The starter's `app/run_tests.rs` contains a bootstrap
  smoke test; extend or replace it as the application grows.

## Development commands

Use Justfile as the interface for development:

- `just fmt` formats the whole workspace.
- `just fmt-check` checks formatting without changing files.
- `just build [package]` builds the workspace or one package in release mode.
- `just lint [package]` runs Clippy with all features and all targets; the whole-workspace form also
  runs `just lint-arch`.
- `just lint-arch` runs the Dylint lints (rust_dylints repository: layout and test style).
- `just test [package]` runs nextest in release mode.
- `just test-doc [package]` runs documentation tests separately from nextest.
- `just run [package] [arguments]` runs an application in release mode.

Do not invoke Cargo directly to build, test or lint. Cargo commands inside
Justfile are the intended implementation; build, lint, test and run use RTK.
After changes run `just fmt`, `just build`, `just lint` and `just test`.
For libraries containing documentation tests also run `just test-doc`.

## Documentation and completion

- Keep README dependency installation and run instructions current, including
  native libraries, external executables, services and optional toolchains.
- Before reporting completion verify member inheritance and changed module layout.
- Verify main.rs, lib.rs and mod.rs contain only the allowed wiring and exports.
- Verify `just lint` passes, including the `layout_*` and `testing_*` lints. If a rule cannot be met,
  do not claim completion; explain the deviation and ask.
- Report checks actually run and any unresolved failures or instruction deviations.
  Do not claim completion when applicable requirements remain unmet.

@RTK.md
