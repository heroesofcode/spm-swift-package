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
                                              ├── PlatformValidator → Package.swift
                                              ├── ProjectFile → file I/O
                                              └── ProjectTemplates → template strings
```

The GUI (`SpmView`) drives `SpmBuilder` directly, bypassing `CliController`.

### Tests

Integration tests live in `crates/cli/tests/`. They create real temporary directories. Key test files:
- `spm_builder_tests.rs` — end-to-end project generation
- `project_templates_tests.rs` — template content assertions
- `platform_validator_tests.rs` — platform config

## CI

GitHub Actions (`.github/workflows/CI.yml`) runs on PRs against macOS, executing `xtask build` then `xtask test`. Releases are triggered by commits to main whose message contains `"Prepare version to"`.
