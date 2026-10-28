---
name: bevy-workspace
description: Create a new Bevy game workspace with Ivan's Cargo metadata, strict Clippy lints, RTK, nextest, Justfile and the enforced Bevy architecture rules (feature crates, leaf/composite plugins, data/input/simulation/presentation layers, Dylint architecture lints). Use when asked to start a new Bevy project or to port these Bevy architecture rules/lints to a project; do not apply to unrelated edits or replace an existing workspace.
---

# Bevy workspace with enforced architecture

Self-contained [templates](templates/) and
[scripts/new_workspace.py](scripts/new_workspace.py). Resolve paths relative to
this skill, never relative to the current project.

```sh
python3 <skill-directory>/scripts/new_workspace.py <destination> \
  --repository <actual-repository-url> [--name <package-name>] \
  [--description <project-description>]
```

Infer destination and name from the request; ask for a missing repository URL
rather than inventing one. The destination must not exist. The script copies
templates and substitutes `__NAME__`, `__NAME_IDENT__` (underscored) and TOML
placeholders; it runs no Cargo, Git or installs.

## What the template contains

- `AGENTS.md` (process, commands) and `docs/BEVY_ARCHITECTURE.md` (the rules, one
  place, no duplicates). Read both in the generated project before implementing.
- Workspace layout: crates use the short prefix `game-` whatever the project is called
  (`game-core`, `game-view`, ...); only the application takes the project name. `crates/game-core` (contract crate: shared types only),
  `crates/game-spin` (reference leaf feature with `data/`, `messaging/`, `infrastructure/`,
  `simulation/`, `presentation/` with a `scenes/` BSN example, headless rstest), `apps/<name>` (composes plugins, orders `GameSet`).
- Dylint libraries `rust_layout` and `rust_testing` (any Rust: layout, rstest style) and `bevy_layout` (Bevy features), consumed
  from https://github.com/pengwin/rust_dylints by git tag in `[workspace.metadata.dylint]`.
  `dylint.toml` holds thresholds, forbidden file names and `exempt_packages`.
- `just lint` = Clippy + `lint-arch` (which includes `layout_crate_dependency`).

## Requirements and cautions

- Needs `cargo-dylint` and `dylint-link` >= 6 (`cargo install cargo-dylint dylint-link --locked`).
  The first `just lint-arch` makes rustup install the pinned nightly from
  the `rust_dylints` repository's `rust-toolchain.toml`. Do not install global tools silently:
  tell the user what is missing. Dylint 5.x pins an older nightly which cannot compile Bevy 0.19.
- The pinned nightly must be at least the workspace `rust-version`; when bumping either,
  run `cargo dylint new` in a scratch directory to read the toolchain the current
  Dylint release pins, then update `rust-toolchain.toml` and fix API drift in the `rust_dylints` repository.
- Bevy API: inspect the real source or example (`.references/bevy`, a checkout of the
  matching tag) before writing Bevy code; the template follows `examples/app/headless.rs`
  and `examples/ecs/ecs_guide.rs`.
- Do not copy game-specific settings from other projects. Character/asset export
  rules are project rules, not part of this template.

## Changing the rules

Rule text lives only here (`templates/docs/BEVY_ARCHITECTURE.md`); the lints live in the
`rust_dylints` repository (https://github.com/pengwin/rust_dylints). A rule and its lint change
together: edit the doc, edit the lint plus its fixtures there (`just test-fixtures`), add a
CHANGELOG entry, tag a release, then bump the `tag` in `templates/Cargo.toml` and the lint
names in the doc, `AGENTS.md` and `dylint.toml`. Verify by generating a workspace and
running `just lint`.

## Verification after generating

Run `just fmt`, `just build`, `just lint`, `just test` and `just run`
in the new workspace; report results, any missing tool, and nothing else installed.
