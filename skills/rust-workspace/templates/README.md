# __NAME__

__DESCRIPTION__

## Dependencies

- Rust >= 1.98.1 and Cargo, installed through rustup. Edition: 2024.
- rustfmt and Clippy: `rustup component add rustfmt clippy`.
- just: `cargo install just --locked`.
- cargo-nextest: `cargo install cargo-nextest --locked`.
- RTK: `cargo install --git https://github.com/rtk-ai/rtk --locked`
  (requires Git for installation), or `brew install rtk` on macOS.
  Official installation instructions: https://github.com/rtk-ai/rtk#installation.
- Bash, used by Justfile recipes. On Windows use an environment with Bash
  and make Rust, just, nextest and RTK available in that same environment.
- A native linker/C toolchain appropriate for the target: Linux C compiler
  and linker (for example build-essential on Debian/Ubuntu), macOS Xcode
  Command Line Tools, or Windows MSVC Build Tools with the Windows SDK.

The starter has no third-party Rust dependencies or runtime system libraries.
Document any added native libraries, external services or tools here, including
optional dependencies and the commands which require them.
Python 3 is needed only to use the workspace generator, not to build this project.

### Installation (Linux / macOS)

Install Rust/Cargo using https://rustup.rs, then:

```sh
rustup component add rustfmt clippy
cargo install just --locked
cargo install cargo-nextest --locked
cargo install --git https://github.com/rtk-ai/rtk --locked
export PATH="$HOME/.cargo/bin:$PATH"
```

On Debian/Ubuntu install the native toolchain, Bash and Python with
`sudo apt install build-essential bash python3 git`.
On macOS install the native toolchain with `xcode-select --install`.
On Windows install Rust with the MSVC toolchain and Visual Studio Build Tools
(C++ workload and Windows SDK), plus Bash (for example Git Bash) and Python 3.
RTK can also be installed with `winget install rtk-ai.rtk`.

Verify executables in the shell used for development:

```sh
rustc --version
cargo --version
cargo fmt --version
cargo clippy --version
just --version
cargo nextest --version
rtk --version
```

The generator does not initialize Git. `.gitignore` excludes `/target/` and
keeps Cargo.lock available for version control.

## Commands

```sh
just fmt
just fmt-check
just build                 # whole workspace, release
just build __NAME__        # one package
just lint                  # Clippy, all targets and features
just test                  # nextest, release
just test-doc              # documentation tests, separately
just run                   # default application, release
just run __NAME__ argument # application arguments
```

Track Cargo.lock when you put this workspace under version control. Metadata and lints are inherited from root Cargo.toml.
Resolver 3 (MSRV-aware: respects `rust-version` when choosing dependency versions) and the lint levels are the defaults of the workspace skills; no custom Cargo profiles,
nextest configuration, rustflags or engine dependencies are included.
RTK wraps Cargo in Justfile. Agent output handling is described in RTK.md.

## Adding packages

```sh
cargo new crates/example-core --lib --vcs none
cargo new tools/example-tool --bin --vcs none
```

Add `crates/*` or `tools/*` to workspace.members when the first package exists.
Replace generated metadata with workspace inheritance as in the existing app,
add `[lints] workspace = true`, and inherit shared dependencies explicitly.
`cargo new` itself does not apply this template or workspace lint inheritance.

## License

The starter uses a proprietary, all-rights-reserved LICENSE placeholder.
Choose and replace LICENSE before distributing the project under another license.

The starter includes a `#[cfg(test)] mod tests` in the application bootstrap,
with a smoke test verifying that startup completes without panicking.
Add tests of real application behavior as the implementation grows.
