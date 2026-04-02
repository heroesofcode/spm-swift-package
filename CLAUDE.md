# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What This Project Does

`spm-swift-package` is a macOS CLI tool (written in Rust) that generates Swift Package Manager projects with sensible defaults. It supports two modes: interactive CLI and GUI (via Iced). Generated projects support iOS, macOS, tvOS, watchOS, and visionOS platforms with optional SwiftLint plugin integration.

## Commands

This project uses [mise](https://mise.jdx.dev/) as a task runner. All tasks are defined in `mise.toml`.

```bash
# Build
cargo build                          # debug build
cargo build --release                # release build

# Run
cargo run                            # interactive CLI
cargo run -- ui                      # GUI mode
cargo run -- generate --name MyLib --platform ios   # non-interactive

# Test
cargo test                           # all tests
cargo test <test_name>               # single test
cargo test -p cli                    # tests for cli crate only

# Lint & Format
cargo clippy --all-targets --all-features   # lint
cargo fmt --all -- --check                  # check formatting
cargo fmt --all                             # apply formatting

# CI equivalent (what GitHub Actions runs)
cargo run -p xtask -- build
cargo run -p xtask -- test
```

### mise tasks

All tasks are also available via `mise run <task>`:

| Task         | Command                          | Description                              |
|--------------|----------------------------------|------------------------------------------|
| `build`      | `cargo build`                    | Debug build                              |
| `release`    | `cargo build --release`          | Release build                            |
| `cli`        | `cargo run`                      | Run interactive CLI                      |
| `ui`         | `cargo run -- ui`                | Run GUI mode                             |
| `test`       | `cargo test`                     | Run all tests                            |
| `lint`       | `cargo clippy --all-targets ...` | Lint                                     |
| `fmt`        | `cargo fmt --all -- --check`     | Check formatting                         |
| `check`      | `cargo check`                    | Check without building artifacts         |
| `doc`        | `cargo doc --no-deps`            | Generate docs                            |
| `spm`        | `cargo run -- generate ...`      | Generate a Swift Package non-interactively (used in CI) |
| `swift`      | `cd MyLib && swift build`        | Build the generated Swift package (used in CI) |
| `changelog`  | `git cliff -o CHANGELOG.md`      | Generate CHANGELOG                       |
| `releasor`   | `releasor -f spm-swift-package`  | Generate `.tar.gz` release artifacts     |
| `publish`    | `cargo publish`                  | Publish to crates.io                     |

## Formatting Rules (`.rustfmt.toml`)

- Hard tabs (not spaces)
- 2-space tab width
- Per-item import granularity (`use foo::bar; use foo::baz;` not `use foo::{bar, baz};`)
- Imports grouped: std → external → crate

## Architecture

The project is a Cargo workspace with two crates:

- `crates/cli/` — main application
- `crates/xtask/` — build automation (build, test, publish tasks)

### `crates/cli/src/` layers

**`core/`** — business logic, no I/O side effects at the type level:
- `spm_builder.rs` — top-level orchestrator; calls platform validator and file writer
- `platform_validator.rs` — maps selected platforms → Package.swift content; handles SwiftLint plugin injection
- `file/project_file.rs` — file system writes
- `file/project_templates.rs` — all Swift file content as `Cow<'static, str>` template functions

**`cli/`** — CLI interaction:
- `cli_controller.rs` — `demand`-based interactive prompts (project name, platform, test framework, optional files), spinner during generation, Xcode launch
- `header.rs` — ASCII banner

**`ui/`** — Iced GUI (`spm_view.rs`); mirrors the CLI workflow in a graphical form, launched via `spm-swift-package ui`

**`utils/`** — shared color theme constants (`theme_colors.rs`)

### Data flow

```
main.rs (clap) → CliController (prompts) → SpmBuilder.build()
               → generate subcommand      ↗ (non-interactive, bypasses prompts)
                                              ├── PlatformValidator → Package.swift
                                              ├── ProjectFile → file I/O
                                              └── ProjectTemplates → template strings
```

The GUI (`SpmView`) and the `generate` subcommand both drive `SpmBuilder` directly, bypassing `CliController`.

### Tests

Integration tests live in `crates/cli/tests/`. They create real temporary directories. Key test files:
- `spm_builder_tests.rs` — end-to-end project generation
- `project_templates_tests.rs` — template content assertions
- `platform_validator_tests.rs` — platform config

## CI

GitHub Actions (`.github/workflows/CI.yml`) runs on PRs on `macos-26`, executing:
1. `xtask build` — compiles the project
2. `xtask test` — runs all tests
3. `mise spm` — generates a Swift Package non-interactively to validate end-to-end output
4. `mise swift` — runs `swift build` on the generated package to confirm it compiles

The Xcode version is controlled by `.xcode-version` at the repo root and applied in CI via the `maxim-lobanov/setup-xcode` action.

Releases are triggered by commits to main whose message contains `"Prepare version to"`.
