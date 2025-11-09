<p align="center">
  <img src="./assets/icon.webp" width="300" />
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

Run the command to start creating your Swift Package Manager:

```sh
spm-swift-package
```

<img src="https://github.com/heroesofcode/spm-swift-package/blob/main/assets/example.gif?raw=true">

After generating the Package, it will automatically open in Xcode

<img src="https://github.com/heroesofcode/spm-swift-package/blob/main/assets/xcode.png?raw=true">

And from here you can continue working on your SPM project 🚀 🙂 👨‍💻 👩‍💻

## Contributing

To contribute, just fork this project and then open a pull request, feel free to contribute, bring ideas and raise any problem in the issue tab.

## License

spm-swift-package is released under the MIT license. See [LICENSE](https://github.com/heroesofcode/spm-swift-package/blob/main/LICENSE) for details.
