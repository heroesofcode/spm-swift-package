# CHANGELOG

## Version 0.10.1
**2025-12-10**

- fix: the UI header is not displayed in Light Mode by @joaolfp (#199)
- refactor: taking version direct cargo by @joaolfp (#197)

## Version 0.10.0
**2025-12-09**

- chore(deps): update iced to 0.14.0 by @joaolfp (#196)
- feat: add rustdocs by @joaolfp (#195)
- fix: fix lint and fmt warnings by @joaolfp (#194)
- fix: fix bugbot cursor commenteds by @joaolfp (#193)
- feat: add GUI support by @joaolfp (#192)
- refactor: update input to args xtask and remove usecase by @joaolfp (#191)
- fix: fix panic input xtask by @joaolfp (#190)
- refactor: indent code and fix lint by @joaolfp (#189)
- chore(deps): update Rust crate insta to v1.44.3 by @renovatebot (#188)
- chore(deps): update Rust crate demand to v1.8.0 by @renovatebot (#187)

## Version 0.9.0
**2025-11-17**

- chore(deps): update Rust crate clap to v4.5.52 by @renovatebot (#175)
- chore(deps): update SwiftLintPlugin to 0.62.2 (was 0.61.0) by @joaolfp (#176)

## Version 0.8.1
**2025-11-14**

- fix: selector platform #173 by @joaolfp (#174)
- chore(deps): update rust to v1.91.1 by @renovatebot (#171)
- chore(deps): update Rust crate clap to v4.5.51 by @renovatebot (#166)

## Version 0.8.0
**2025-10-19**

- refactor: change SwiftLint mise to Plugin by @joaolfp (#164)
- chore(deps): update Rust crate clap to v4.5.49 by @renovatebot (#160)
- chore(deps): update Rust crate httpmock to v0.8.2 by @renovatebot (#163)
- chore(deps): update Rust crate tokio to v1.48.0 by @renovatebot (#162)
- chore(deps): update Rust crate serde to v1.0.228 by @renovatebot (#159)
- chore(deps): update rust to v1.90.0 to v1.90.0 by @renovatebot (#152)

## Version 0.7.0
**2025-09-17**

- refactor: update platform versions to Xcode 26 by @joaolfp - #151
- feat: xcode 26.0 support by @joaolfp - #149
- chore(deps): migrate renovate config by @renovatebot - #144
- chore(deps): update Rust crate insta to v1.43.2 by @renovatebot - #143
- chore(deps): update Rust crate serde to v1.0.225 by @renovatebot - #150
- chore(deps): update Rust crate clap to v4.5.47 by @renovatebot - #142
- chore(deps): update jdx/mise-action action to v3 by @renovatebot - #140

This release introduces support for Xcode 26.0. If you are not ready to migrate to Xcode 26 yet, I recommend that you either stay on your current version or avoid installing this release of `spm-swift-package`.

## Version 0.6.0
**2025-08-15**

🚜 Refactor
- refactor: update swift tools version to 6.1 by @joaolfp - #130

📦️ Dependency Updates
- update Rust crate async-trait to v0.1.89 by @renovatebot - #139
- update Rust crate clap to v4.5.45 by @renovatebot - #138
- update Rust crate reqwest to v0.12.23 by @renovatebot - #137
- update Rust to v1.89.0 by @renovatebot - #134
- update Rust crate tokio to v1.47.1 by @renovatebot - #132

## Version 0.5.1
**2025-04-22**

🐛 Bug Fixes
- clear screen input by @joaolfp - #73

🚜 Refactor
- refactor: adjust variable name and project by @joaolfp - #72
- removing some files by @joaolfp - #70

📦️ Dependency Updates
- update Rust crate clap to v4.5.37 by @renovatebot - #71

## Version 0.5.0
**2025-04-16**

🚀 Features
- create the data folder for dto by @joaolfp - #68
- migrate to clean architecture by @joaolfp - #67
- migrate project to layered architecture by @joaolfp - #66
- add rust-toolchain.toml by @joaolfp - #65

🚜 Refactor
- rename parameters and organization code by @joaolfp - #69
- network improvement by @joaolfp - #41

📦️ Dependency Updates
- update Rust crate clap to v4.5.36 by @renovatebot - #64
- update openssl from 0.10.70 to 0.10.72 by @dependabot - #63
- update Rust crate clap to v4.5.35 by @renovatebot - #61
- update Rust crate clap to v4.5.34 by @renovatebot - #60
- update Rust crate reqwest to v0.12.15 by @renovatebot - #59
- update Rust crate demand to v1.6.5 by @renovatebot - #58
- update Rust crate tokio to v1.44.1 by @renovatebot - #57
- update Rust crate reqwest to v0.12.14 by @renovatebot - #56
- update Rust crate reqwest to v0.12.13 by @renovatebot - #55
- update Rust crate clap to v4.5.32 by @renovatebot #54
- update Rust crate serde to v1.0.219 by @renovatebot - #53
- bump ring from 0.17.8 to 0.17.13 by @dependabot - #52
- update Rust crate tokio to v1.44.0 by @renovatebot - #51
- update update Rust crate insta to v1.42.2 by @renovatebot - #50
- update Rust crate demand to v1.6.4 by @renovatebot - #49
- update update Rust crate serde to v1.0.218 by @renovatebot - #47
- update Rust crate demand to v1.6.3 by @renovatebot - #46 
- update Rust crate clap to v4.5.30 by @renovatebot - #45
- update Rust crate clap to v4.5.29 by @renovatebot - #44
- bump openssl from 0.10.68 to 0.10.70 by @dependabot - #43
- update Rust crate clap to v4.5.28 by @renovatebot - #42

## Version 0.4.2
**2025-01-26**

🐛 Bug Fixes
- when user doesn't select any of the options in multiple selection by @joaolfp - #40

📦️ Dependency Updates
- update Rust crate insta to v1.42.1 by @renovatebot - #38
- update Rust crate demand to v1.6.2 by @renovatebot - #37
- update Rust crate clap to v4.5.27 by @renovatebot - #36

## Version 0.4.1
**2025-01-02**

🐛 Bug Fixes
- control c is generating an interrupt error in multi select by @joaolfp - #26

📚 Documentation
- update example gif by @joaolfp - #23

📦️ Dependency Updates
- update Rust crate reqwest to v0.12.12 by @renovatebot - #25

## Version 0.4.0
**2024-12-30**

🚀 Features
- option choose platform by @joaolfp - #22
- add option swiftlint with mise by @joaolfp - #19

📚 Documentation
- remove msrv badge by @joaolfp - #21

## Version 0.3.0
**2024-12-28**

🚀 Features
- update mise by @joaolfp - #16

🐛 Bug Fixes
- control c input library by @joaolfp - #18
- lint warning by @joaolfp - #17

🧪 Testing
- add unit tests by @joaolfp - #15

📦️ Dependency Updates
- update Rust crate demand to v1.6.0 by @renovatebot - #14
- update Rust crate clap to v4.5.23 by @renovatebot - #10

## Version 0.2.0
**2024-11-05**

- Update header and adjust readme by @joaolfp - #7
- Add option to Swift Package Index and adjust documentation by @joaolfp - #6
- Add mise and fix lint by @joaolfp - #4

## Version 0.1.0
**2024-10-21**

- Add conf files and create readme by @joaolfp - #3
- Create command structure by @joaolfp - #2
- Create base projet by @joaolfp - #1
