<p align="center">
  <img src="https://raw.githubusercontent.com/heroesofcode/spm-swift-package/refs/heads/main/assets/icon.webp" width="300" />
  <br>

  <a href="https://github.com/heroesofcode/spm-swift-package/releases">
    <img src="https://img.shields.io/github/v/release/heroesofcode/spm-swift-package?style=flat&labelColor=1C2C2E&color=C96329&logo=GitHub&logoColor=white" /></a>
    
  <a href="https://crates.io/crates/spm-swift-package">
    <img src="https://img.shields.io/crates/v/spm-swift-package?style=flat&labelColor=1C2C2E&color=C96329&logo=Rust&logoColor=white" /></a>
    
  <a href="https://github.com/heroesofcode/spm-swift-package/actions/workflows/CI.yml">
    <img src="https://img.shields.io/github/actions/workflow/status/heroesofcode/spm-swift-package/CI.yml?style=flat&labelColor=1C2C2E&label=CI&color=BEC5C9&logo=GitHub%20Actions&logoColor=BEC5C9" /></a>
    
  <a href="https://github.com/heroesofcode/spm-swift-package/actions/workflows/Release.yml">
    <img src="https://img.shields.io/github/actions/workflow/status/heroesofcode/spm-swift-package/Release.yml?style=flat&labelColor=1C2C2E&color=BEC5C9&label=deploy&logo=GitHub%20Actions&logoColor=BEC5C9" /></a>
    
  <a href="https://github.com/heroesofcode/spm-swift-package/blob/main/LICENSE">
    <img src="https://img.shields.io/badge/license-MIT-BEC5C9?style=flat&labelColor=1C2C2E&logoColor=BEC5C9" /></a>
</p>

<p align="center">
  <strong>Command Line Tools for macOS to create Swift Package Manager projects with desirable files.</strong>
</p>

## Features
- 🚀 **Multi-platform:** iOS • macOS • tvOS • watchOS • visionOS  
- 🛠️ **Auto-generated files:** Changelog • Readme • Swift Package Index • SwiftLint  
- ✅ **SwiftLint integration:** Generates `.swiftlint.yml` using the [SwiftLintPlugin](https://github.com/lukepistrol/SwiftLintPlugin)  
- 🧰 **Modern toolchain:** Compatible with Xcode 26.0  
- ⚠️ **No legacy support:** Does not work with older Xcode versions  
- 🖥️ **GUI support:** Optional graphical interface built with [Iced](https://github.com/iced-rs/iced), launched via `spm-swift-package ui`

## Installing

#### Cargo 🦀
Installing from [crates.io](https://crates.io/) (requires Rust/Cargo):

```shell
cargo install spm-swift-package
```

#### Homebrew 🍻
You can install with [Homebrew](https://brew.sh/):

```shell
brew tap heroesofcode/taps
brew install heroesofcode/taps/spm-swift-package
```

## Usage

#### Run CLI

```sh
spm-swift-package
```

<img src="https://raw.githubusercontent.com/heroesofcode/spm-swift-package/main/assets/example.gif">

#### Generate (non-interactive)

Generate a package without any prompts, useful for CI/automation:

```sh
spm-swift-package generate --name MyLib --platform ios
```

| Flag | Short | Description | Required |
|------|-------|-------------|----------|
| `--name` | `-n` | Package name | Yes |
| `--platform` | `-p` | Target platform: `iOS`, `macOS`, `tvOS`, `watchOS`, `visionOS` | Yes |
| `--test-framework` | `-t` | Test framework: `xctest` (default), `swift-testing` | No |
| `--files` | `-f` | Optional files (repeatable): `changelog`, `readme`, `spi`, `swiftlint` | No |
| `--open-xcode` | | Open the package in Xcode after generation | No |

Example with all options:

```sh
spm-swift-package generate \
  --name MyLib \
  --platform ios \
  --test-framework swift-testing \
  --files changelog \
  --files readme \
  --open-xcode
```

#### Run UI

```sh
spm-swift-package ui
```

<img src="https://raw.githubusercontent.com/heroesofcode/spm-swift-package/main/assets/gui.png">

<a href="https://github.com/iced-rs/iced">
  <img src="https://gist.githubusercontent.com/hecrj/ad7ecd38f6e47ff3688a38c79fd108f0/raw/74384875ecbad02ae2a926425e9bcafd0695bade/color.svg" width="130px">
</a>

<br>
<br>

After generating the Package, it will automatically open in Xcode

<img src="https://raw.githubusercontent.com/heroesofcode/spm-swift-package/main/assets/xcode.png">

And from here you can continue working on your SPM project 🚀 🙂 👨‍💻 👩‍💻

## Contributing

To contribute, just fork this project and then open a pull request, feel free to contribute, bring ideas and raise any problem in the issue tab.

## License

spm-swift-package is released under the MIT license. See [LICENSE](https://github.com/heroesofcode/spm-swift-package/blob/main/LICENSE) for details.
