# skillz

A [Claude Code](https://claude.com/claude-code) plugin with skills that start Rust projects the way
I like them: strict Clippy, a `Justfile`, nextest, and, for Bevy, architecture rules enforced by the
[Dylint](https://github.com/trailofbits/dylint) lints of
[rust_dylints](https://github.com/pengwin/rust_dylints).

| Skill | Creates |
|---|---|
| `skillz:rust-workspace` | a Rust workspace: shared Cargo metadata and lints, `apps/`, `crates/`, RTK, cargo-nextest, `Justfile`, `AGENTS.md` |
| `skillz:bevy-workspace` | a Bevy game workspace on top of that: feature crates (`game-*`), leaf and composite plugins, `data/`, `messaging/`, `infrastructure/`, `input/`, `simulation/`, `presentation/` layers, the `rust_dylints` architecture lints and `docs/BEVY_ARCHITECTURE.md` |

## Install

```text
/plugin marketplace add pengwin/skillz
/plugin install skillz@pengwin-skillz
```

Then ask Claude for a new workspace ("start a Bevy project called ... in ~/code/...") or call a skill
by name. Each skill is self-contained: `skills/<name>/templates/` and `skills/<name>/scripts/new_workspace.py`
(Python 3, no other dependencies). The script copies the templates and substitutes the project name, runs no
Cargo, Git or installs.

The generated Bevy workspace needs the tools listed in its README, among them `cargo-dylint` and `dylint-link`
(version 6 or newer) for `just lint`.

## Layout

```text
.claude-plugin/plugin.json        the plugin
.claude-plugin/marketplace.json   makes this repository installable as a marketplace
skills/rust-workspace/
skills/bevy-workspace/
```

## Changing a skill

The rule text of the Bevy architecture lives only in `skills/bevy-workspace/templates/docs/BEVY_ARCHITECTURE.md`,
the lints that enforce it in [rust_dylints](https://github.com/pengwin/rust_dylints). A rule and its lint change
together; then bump the `tag` in `skills/bevy-workspace/templates/Cargo.toml`.

## License

[MIT](LICENSE). The files under `skills/*/templates/` are templates for new projects; the `LICENSE` they contain
is a placeholder for the project that is generated, not a license of this repository.
