# Bevy workspace

This is a Rust workspace for a game built on the Bevy engine (0.19).

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

## Bevy architecture

Follow [docs/BEVY_ARCHITECTURE.md](docs/BEVY_ARCHITECTURE.md) completely before
creating or changing any feature. In short: features are crates; a feature is a
leaf (`plugin.rs` + layers `data/`, `messaging/`, `infrastructure/`, `input/`,
`simulation/`, `presentation/`) or a composite of child plugins; features share types
only through contract crates; plain `pub` only.

Placing new code in a leaf: components and resources go in `data/`, `Message`/`Event`
types in `messaging/`, `SystemSet` types and all wiring/loading/configuration that is not
game logic in `infrastructure/` (also `SystemParam` types shared by systems of several
layers; a `SystemParam` used by one system stays next to it), controls in `input/`, game rules in `simulation/`,
anything the player sees in `presentation/` (scenes in `presentation/scenes/`). Custom assets: the `Asset` type goes in `data/`, the `AssetLoader` in `infrastructure/`, and
`init_asset`/`init_asset_loader` are called from `plugin.rs`. A feature with a non-trivial asset
pipeline gets a child plugin `assets/` instead of growing its own.
The crate root `src/` holds only `lib.rs`/`main.rs` (plus `plugin.rs` in a feature crate) and
module directories: a module is `foo/mod.rs` with declarations only and its code in files beside it.
Spawning: describe entities as scenes (`bsn!`, `bsn_list!`, `#[derive(SceneComponent)]`) in
`presentation/scenes/` and spawn them with `spawn_scene`/`queue_spawn_scene`/`.spawn()`. Prefer a
scene over several consecutive `Commands::spawn` or `.with_children` calls; compose small scene
functions and patch fields instead of copying them. Scenes outside `presentation/scenes/` are a lint error.
Systems read freely and write little: at most one `ResMut` and two mutable component
accesses per system (counted through `ParamSet`, tuples and `SystemParam` bundles), and no
`&mut World` systems outside `infrastructure/`. If a system needs more, split it or change the
data model; never wrap parameters or raise limits to silence the lint.
Every `mod.rs`, `lib.rs` and `plugin.rs`
starts with a `//!` comment (at least three words) saying what the module owns; write it
when you create the file and update it when the module's responsibility changes. Flat files such as `data.rs`, `systems.rs`, `components.rs` are
prohibited. `just lint` enforces these rules; never silence an `layout_*` or `bevy_*` lint
with a crate-wide allow, fix the structure instead.

Before writing or changing code that uses Bevy APIs, inspect the relevant Bevy
source or the closest official example (clone `https://github.com/bevyengine/bevy`
at the tag matching the dependency into `.references/bevy`), prefer it over
remembered APIs, and mention which example materially shaped the code.

## Rust code design

- Keep main.rs, lib.rs and mod.rs limited to declarations, module wiring and
  intended re-exports. Allow minimum entry-point delegation in main.rs.
- Put business logic in separate implementation files.
- Keep implementation details private. Exposure is decided by lib.rs and mod.rs
  re-exports; pub(crate), pub(super) and pub(in ...) are not used.
- Write tests with rstest: `#[rstest]` (never plain `#[test]`), shared setup in `#[fixture]`s, variants as `#[case]`.
- Add tests for application behavior. The starter's `#[cfg(test)] mod tests`
  contains a bootstrap smoke test; extend or replace it as the application grows.

## Development commands

Use Justfile as the interface for development:

- `just fmt` formats the whole workspace.
- `just fmt-check` checks formatting without changing files.
- `just build [package]` builds the workspace or one package in release mode.
- `just lint [package]` runs Clippy with all features and all targets; the
  whole-workspace form also runs `just lint-arch`.
- `just lint-arch` runs the Dylint architecture lints (rust_dylints repository),
  including crate dependency direction.
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
- Verify `just lint` passes, including the `layout_*` or `bevy_*` lints. If an architecture rule
  cannot be met, do not claim completion; explain the deviation and ask.
- Report checks actually run and any unresolved failures or instruction deviations.
  Do not claim completion when applicable requirements remain unmet.

@RTK.md
