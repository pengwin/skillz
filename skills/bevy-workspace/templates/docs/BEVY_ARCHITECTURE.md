# Bevy architecture rules

Architecture is enforced mechanically: `just lint` runs Clippy and the Dylint libraries `rust_layout` and `bevy_layout` from
https://github.com/pengwin/rust_dylints (`just lint-arch`). A rule without a lint is
marked *(review)*. Lint names are in backticks; configure thresholds in `dylint.toml`.

## 1. Features are crates

- Every cohesive feature is its own crate in `crates/`. The crate root is the
  feature: it declares `plugin` and either layers (leaf) or child features (composite).
- Contract crates (`*-core`, listed in `contract_crates` of `dylint.toml`) hold
  types shared between features. A feature crate depends on contract crates and
  never on another feature crate (`layout_crate_dependency`, `contract_crates` in `dylint.toml`).
- Applications in `apps/` compose feature plugins and configure schedules.
  Tools, benchmarks and generated-schema crates are not features: list them in
  `exempt_packages`.

## 2. A feature is a leaf or a composite, never both

| Node | Contains | `plugin.rs` does |
|---|---|---|
| Leaf | `plugin.rs` + layer directories (`data/`, `messaging/`, `infrastructure/`, `input/`, `simulation/`, `presentation/`; only those needed) | registers the feature's systems |
| Composite | `plugin.rs` + child features (each with its own `plugin.rs`) | only `add_plugins((A, B, ...))` and configuration |

- `bevy_mixed_feature`: a node with layers and child features is rejected.
- `bevy_unlayered_module`: a leaf may not hold stray modules (`loader.rs`,
  `helpers.rs`); put code in a layer or extract a child feature.
- `bevy_plugin_too_large`: a leaf plugin registers at most `max_registered_systems`
  (6) systems. `bevy_data_too_wide`: `data/` has at most `max_data_modules` (8)
  concept modules. Both mean the same thing: split the feature into child plugins.
- Test scaffolding (`tests.rs`, `*_tests.rs`, `test_support.rs`, `#[cfg(test)]`
  modules) may sit next to the code.

## 3. Layers of a leaf

Six layer directories, all optional (create only what the feature needs). Three are the
**foundation** every other layer may import; three hold the **systems**.

| Layer | Owns | May import |
|---|---|---|
| `data/` | components, resources, intents; types only | nothing from other layers |
| `messaging/` | `Message`/`Event` types the feature writes and reads; types only | `data/` |
| `infrastructure/` | everything that is not game logic: `SystemSet` types, schedule wiring, asset loaders, config loading, plugin plumbing | `data/`, `messaging/` |
| `input/` | reads controls, applies input gating, writes intents | foundation |
| `simulation/` | game rules; changes simulation state only; must run headless | foundation |
| `presentation/` | spawns and changes what the player sees (meshes, materials, transforms, UI, effects) | foundation, `simulation/` |

- `bevy_layer_dependency`: imports against the table, and rendering imports
  (`bevy::pbr`, `bevy::render`, `bevy::ui`, ...) in `data/`, `messaging/`, `input/`,
  `simulation/`. Rendering plumbing may live in `infrastructure/` and `presentation/`.
- `bevy_misplaced_type`: `SystemSet` types live in `infrastructure/`, `Message`/`Event`
  types in `messaging/`. Contract crates are exempt (they hold shared sets and messages).
- `SystemParam` bundles and `ParamSet`: a `#[derive(SystemParam)]` type is a borrow bundle for
  systems, not data. Keep it private next to the system that uses it (`input/`, `simulation/`,
  `presentation/`); if systems of several layers share it, put it in `infrastructure/`. It is
  rejected in `data/` and `messaging/` (`bevy_misplaced_type`) and never crosses a feature
  boundary. `ParamSet` is not declared anywhere: it is a parameter in the system's signature.
- `bevy_system_in_data`: `data/` and `messaging/` contain no systems (functions with
  system parameters).
- `bevy_flat_layer` / `layout_forbidden_file`: layers are directories; `data.rs`,
  `input.rs`, `components.rs`, `systems.rs` and similar are rejected.
- Deciding the layer: if the result changes what the rules of the game compute
  (hit detection, aim direction used by gameplay, movement), it is `simulation/`.
  `presentation/` only applies results. A feature must keep working in a headless
  test without `presentation/`. Anything that merely wires, loads, orders or configures
  is `infrastructure/`, not `simulation/`.
- Layers talk through foundation types: `input/` writes intents, `simulation/` reads
  them and writes state and messages, `presentation/` reads state and messages.

### Scenes (BSN)

Bevy 0.19 spawns entity groups from scenes: `bsn!`/`bsn_list!` build a `Scene`/`SceneList`
(a composable patch of components, children and observers), and `#[derive(SceneComponent)]`
ties a component to the scene that must accompany it. References: `crates/bevy_scene` and
`examples/scene/bsn.rs` in the Bevy sources.

- Scenes live in `presentation/scenes/` of the feature (`bevy_misplaced_scene`): functions
  returning `impl Scene`/`impl SceneList`/`Box<dyn Scene>`, and `Scene`/`SceneList`/`SceneComponent`
  implementors, including `#[derive(SceneComponent)]`. A `SceneComponent` therefore lives in
  `presentation/scenes/` too, not in `data/`; a plain component that a scene merely uses stays in `data/`.
- Prefer a scene to several consecutive `Commands::spawn` / `.with_children` calls whenever
  you spawn more than one component group or any hierarchy: compose it from small scene
  functions, override single fields by patching, and spawn it with `commands.spawn_scene(..)`,
  `queue_spawn_scene(..)` (waits for asset dependencies) or `scene_fn.spawn()` in `Startup`.
- Components used in `bsn!` need `Default + Clone`, or `#[derive(FromTemplate)]` when a field is an
  asset handle or an entity reference. Assets are referenced by path inside the scene.
- Scene spawning needs `AssetPlugin` and `ScenePlugin` (part of `DefaultPlugins`). Bevy 0.19 has no
  `.bsn` file loader yet, and glTF scenes still use `bevy::world_serialization` (`WorldAssetRoot`).
- Observers declared with `on(..)` inside a scene count as behavior: keep them thin and delegate to
  systems in the feature.

### Systems and parallelism

Bevy runs systems in parallel only when their data access does not conflict, so a system that
writes a lot blocks many others. Read freely, write little.

- `bevy_system_write_access`: a system takes at most `max_resource_writes` (1) `ResMut`-style
  parameters and at most `max_mutable_component_access` (2) mutable component accesses
  (`&mut T`, `Mut<T>` in query data). Tuples, `ParamSet` and crate-local `SystemParam` bundles are
  counted, so wrapping parameters does not get around the limit.
- When the limit is hit, change the design: split the system by what it writes, move state into
  components, or send a message to the owner of the state. Do not wrap parameters to hide them,
  and do not raise the limits or add `allow` to make the lint pass.
- `bevy_exclusive_system`: systems taking `&mut World` block every other system. Only
  `infrastructure/` may have them (setup, teardown, loaders).
- Narrow queries with `With`/`Without` filters, prefer `&T`/`Res<T>`, and check real conflicts with
  Bevy's schedule ambiguity detection and a profiler. *(review)*

### Assets

- A custom asset type (`#[derive(Asset)]`) is data: it lives in `data/` of the feature that owns it,
  or in a contract crate when several features use it.
- Its `AssetLoader`, validation, settings and registration (`init_asset`, `init_asset_loader`) are
  infrastructure: loaders in `infrastructure/` (`bevy_misplaced_type`), registration calls in
  `plugin.rs` only (`bevy_registration_outside_plugin`).
- When a feature has a non-trivial asset pipeline (own loader, dependencies, hot reload,
  validation), make it a child plugin `assets/` with its own `plugin.rs`; the feature becomes a
  composite. One simple asset stays inside its feature. *(review)*

## 4. Organizing `data/`

- Group by domain concept, not by Rust kind. `health.rs` holds `Health`, the
  `Damaged` message and `HealthSet` together, because one concept changes together.
  Do not create `components/`, `resources/`, `messages/` buckets.
- One concept per file. `layout_oversized_file`: a data file over
  `max_data_file_lines` (200) becomes a directory `data/health/` with a `mod.rs`.
- Types the feature exposes to others are re-exported from the crate root.

## 5. Contracts between features

- A type needed by two sibling features moves to a contract crate. Until then it
  stays private to its owner. Do not import a sibling feature, do not widen
  visibility "for convenience".
- Contract crates contain types, constants and system sets only: no systems, no
  plugins with logic. Split contracts by domain (`combat-core`, `character-core`)
  instead of growing one `core`. *(review)*
- Phases of a frame (`GameSet::{Input, Simulation, Presentation}`) live in the
  core contract crate. Features attach systems to a phase; the application bootstrap
  orders phases. Features never order themselves against other features' systems.

## 6. Visibility

- Use plain `pub` or private. What a crate exposes is decided by `lib.rs`
  re-exports, and what a module exposes by its `mod.rs` re-exports.
- `layout_restricted_visibility`: `pub(crate)`, `pub(super)` and `pub(in ...)`
  are rejected. They hide an unclear boundary; fix the module structure instead.

## 7. Registration

- `bevy_registration_outside_plugin`: `impl Plugin` and `add_systems` live in
  `plugin.rs` only, and a composite's `plugin.rs` registers no systems.
- Plugins are small and composable: a parent plugin `add_plugins` its children;
  applications add the top-level plugins.

## 8. Files

- `layout_root_file_logic`: `lib.rs`, `mod.rs` and `main.rs` hold module
  declarations and re-exports only (a one-statement `fn main` may delegate).
- `layout_oversized_file`: source files stay below `max_file_lines` (300).
- `layout_flat_crate_root`: `src/` holds only `lib.rs` (or `main.rs`), `plugin.rs` in a feature
  crate, and module directories. A module is `foo/mod.rs` (declarations only) with its
  implementation files beside it, never a lone `src/foo.rs`.
- `layout_missing_module_doc`: every `mod.rs`, `lib.rs` and `plugin.rs` starts with a `//!`
  comment of at least `min_doc_words` (3) words stating what the module owns. This is the
  contract that tells readers and agents where code belongs; keep it true when the module changes.
- Generated code is excluded through `ignored_path_fragments` (`_generated.rs`).

## 9. Exceptions

- Narrow, documented `#[cfg_attr(dylint_lib = "bevy_layout", allow(bevy_...))]` (or `rust_layout` / `layout_...`) on the offending item; never crate-wide.
- Whole packages which are not Bevy features go to `exempt_packages` in `dylint.toml`.
- Imports through a glob prelude (`use bevy::prelude::*`) are not checked by
  `bevy_layer_dependency`; avoid preludes for rendering types in render-free layers. *(review)*

## 10. Tests

- `testing_plain_test`: tests are `#[rstest]`, never plain `#[test]`.
- `testing_helper_fn`: test code (`#[cfg(test)]` modules, `*_tests.rs`, `test_support.rs`,
  `tests/`) has no plain helper functions. Shared setup is a `#[fixture]` taken as an argument.
- `testing_inline_arrange`: do not build test data (`Type::new(...)`, struct literals) with
  `let` inside a test body; make it a `#[fixture]`.
- Tests which differ only in their input are one `#[rstest]` with `#[case]` arguments. *(review)*
- Simulation tests run headless (`TimePlugin`/`MinimalPlugins`), see
  `crates/game-spin/src/simulation/advance_tests.rs`.
- Dylint only sees test code with `--all-targets`; `just lint-arch` passes it.

## 11. Bevy style under the strict Clippy levels

- Write system parameters with explicit lifetimes (`Query<'_, '_, ...>`,
  `Res<'_, T>`) to satisfy `rust_2018_idioms`.
- Systems take `Res`/`Query` by value: mark each system with
  `#[expect(clippy::needless_pass_by_value, reason = "Bevy system parameters")]`.

## 12. Changing a lint

Lints live in the `rust_dylints` repository (README: adding a lint, fixtures, toolchain
bumps). Change a rule here and its lint there together, release a tag, then bump the
`tag` in `[workspace.metadata.dylint]` of the workspace.
