[![EcoCode on NuGet](https://img.shields.io/nuget/v/EcoCode.svg)](https://www.nuget.org/packages/EcoCode/) [![EcoCode on NuGet](https://img.shields.io/nuget/dt/EcoCode)](https://www.nuget.org/packages/EcoCode/)

EcoCode-Rust

_ecoCode_ is a collective project aiming to reduce environmental footprint of software at the code level. The goal of the project is to provide a list of static code analyzers to highlight code structures that may have a negative ecological impact: energy and resources over-consumption, "fatware", shortening terminals' lifespan, etc.

_ecoCode_ is based on evolving catalogs of [good practices](https://github.com/green-code-initiative/ecoCode/blob/main/docs/rules), for various technologies.

This set of [Clippy](https://github.com/rust-lang/rust-clippy) linters implements these catalogs as rules for scanning your Rust projects using [DyLint](https://github.com/trailofbits/dylint).

> ⚠️ 🚧 This is still a very early stage project 🚧. Any feedback or contribution will be highly appreciated. Please refer to the contribution section.

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0) [![Contributor Covenant](https://img.shields.io/badge/Contributor%20Covenant-2.1-4baaaa.svg)](https://github.com/green-code-initiative/ecoCode-common/blob/main/doc/CODE_OF_CONDUCT.md)

🚀 Getting Started
------------------

[Follow the get started file](getstarted.md)

🧩 Compatibility
-----------------
 
🚧 TODO 🚧

🌿 Rules
-------------------

|Id|Description|Severity|Enabled|
|--|-----------|:------:|:--------:|:------:|
|[EC2](https://github.com/green-code-initiative/ecoCode/blob/main/ecocode-rules-specifications/src/main/rules/EC2/EC2.json)|Avoid multiple if-else statement|⚠️|❌|

🤝 Contribution
---------------

See [contribution](https://github.com/green-code-initiative/ecoCode#-contribution) on the central ecoCode repository.

🤓 Main contributors
--------------------

See [main contributors](https://github.com/green-code-initiative/ecoCode#-main-contributors) on the central ecoCode repository.
