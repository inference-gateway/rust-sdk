## [0.5.1](https://github.com/inference-gateway/rust-sdk/compare/0.5.0...0.5.1) (2025-01-28)

### 🔧 Miscellaneous

* Update README to replace Role with MessageRole for consistency ([f491774](https://github.com/inference-gateway/rust-sdk/commit/f4917745d9e8e993fa3b2a6b2475453f99eb999a))

## [0.5.0](https://github.com/inference-gateway/rust-sdk/compare/0.4.0...0.5.0) (2025-01-28)

### ✨ Features

* Implement custom error handling for Inference Gateway API ([ab8c732](https://github.com/inference-gateway/rust-sdk/commit/ab8c732921f7da799e673f529575f24e7e809135))

### 📚 Documentation

* Add static analysis step to CONTRIBUTING.md ([8000794](https://github.com/inference-gateway/rust-sdk/commit/8000794e8ba98f5f1c92060e11467e1d983adb79))
* **openapi:** Download the latest openapi spec from inference-gateway ([c49a9e6](https://github.com/inference-gateway/rust-sdk/commit/c49a9e6fab6293bc025ff10a2eaeddc66d4b1d38))
* Update CONTRIBUTING.md to reflect new branch naming and task commands ([0196eef](https://github.com/inference-gateway/rust-sdk/commit/0196eefba4f050e17861c71212bb0022619115b3))

### 🔧 Miscellaneous

* Rename Role to MessageRole enum for consistency with openapi ([f97d44b](https://github.com/inference-gateway/rust-sdk/commit/f97d44bdc26348518561551285872fc22af6a065))

## [0.4.0](https://github.com/inference-gateway/rust-sdk/compare/0.3.0...0.4.0) (2025-01-28)

### ✨ Features

* Add get_provider_models method to InferenceGatewayAPI and implement it in InferenceGatewayClient ([92565a7](https://github.com/inference-gateway/rust-sdk/commit/92565a73d8842eb7cfd6a323166bd24699f6ea1d))
* Introduce Role enum for message roles and update usage in README and Message struct ([6bcfa81](https://github.com/inference-gateway/rust-sdk/commit/6bcfa8175a973b7747338a6d2ff36a68d9e276d1))

### 🐛 Bug Fixes

* Simplify provider string formatting in get_provider_models method ([e069fad](https://github.com/inference-gateway/rust-sdk/commit/e069fada1405091462f627fd7e555e4e2efe1d8d))

### 📚 Documentation

* Add CONTRIBUTING.md with guidelines for contributing to the Inference Gateway Rust SDK ([e649871](https://github.com/inference-gateway/rust-sdk/commit/e6498711fdf2f9c018359ba1a7151efc36957934))
* **fix:** Update homepage link in Cargo.toml and README.md to reflect new repository URL ([b93424a](https://github.com/inference-gateway/rust-sdk/commit/b93424a0b116ff59de51709893207b3d34e5ea85))
* **openapi:** Download the latest OpenAPI specification ([fa1edf6](https://github.com/inference-gateway/rust-sdk/commit/fa1edf62598d3192f9d69dae90819bf6da27a5f6))
* Update README.md to include examples for listing models and models by provider ([7d44bb3](https://github.com/inference-gateway/rust-sdk/commit/7d44bb3463be959465c1a1c3226f7615af3b0dbf))

### 📦 Miscellaneous

* Add lint, analyse, and test tasks to Taskfile for improved code quality ([50461a3](https://github.com/inference-gateway/rust-sdk/commit/50461a3d7b3f3769a26fef1e4c0c1d0f283d7fda))

## [0.3.0](https://github.com/inference-gateway/rust-sdk/compare/0.2.5...0.3.0) (2025-01-22)

### ✨ Features

* Add Anthropic provider to the Provider enum ([30004a5](https://github.com/inference-gateway/rust-sdk/commit/30004a591815f06b6c56a1c3f647d5504ac6bcfa))
* Add optional token support for InferenceGatewayClient requests ([9145eb7](https://github.com/inference-gateway/rust-sdk/commit/9145eb785bb4ea51ddcb31e33b233c6a0f585d62))
* Implement InferenceGatewayAPI trait for InferenceGatewayClient ([dc47832](https://github.com/inference-gateway/rust-sdk/commit/dc47832afcb50181b4c6f547e7aeb1a8062d6638))

### 👷 CI

* Enhance release workflow with error handling for actual run of semantic-release ([34279b8](https://github.com/inference-gateway/rust-sdk/commit/34279b8931ddf5e00fb7721d6bcf91811c154261))
* Fix syntax in release workflow for semantic-release execution ([3b94bbc](https://github.com/inference-gateway/rust-sdk/commit/3b94bbc6b07d876567395bc5c5699e9b9a2f581a))
* Remove debug output from release workflow ([a7d03ad](https://github.com/inference-gateway/rust-sdk/commit/a7d03ad97962e68afaf38ba4ed73d1a05f2f6cf8))

### 📚 Documentation

* Enhance InferenceGatewayAPI with health check and detailed documentation ([c70a9e1](https://github.com/inference-gateway/rust-sdk/commit/c70a9e178dae7fd311f100e41fc0b0e98cfd12c4))

## [0.2.5](https://github.com/inference-gateway/rust-sdk/compare/0.2.4...0.2.5) (2025-01-21)

### 👷 CI

* Improve version extraction from dry run output in release workflow ([728b100](https://github.com/inference-gateway/rust-sdk/commit/728b100a26bd7de70f0ed86d5a754eede1b68161))

## [0.2.4](https://github.com/inference-gateway/rust-sdk/compare/0.2.3...0.2.4) (2025-01-21)

### 👷 CI

* Fix dry-run output handling in release workflow ([f822fda](https://github.com/inference-gateway/rust-sdk/commit/f822fda4b9948c91ea6850f6c948dd82eb415a02))
* Refactor release workflow to improve dry-run output handling ([79999b4](https://github.com/inference-gateway/rust-sdk/commit/79999b4cf1a8bb76ca1e1f765b2b8f4305c4e998))
* Update semantic-release version and improve release condition checks ([813a6ff](https://github.com/inference-gateway/rust-sdk/commit/813a6ff53288f208565ffeeaabf2f33e3a9af285))

## [0.2.3](https://github.com/inference-gateway/rust-sdk/compare/0.2.2...0.2.3) (2025-01-21)

### 👷 CI

* Add debug output to release workflow for better visibility ([2264a48](https://github.com/inference-gateway/rust-sdk/commit/2264a4845da865c605707e1b04c9d1377b140613))

## [0.2.2](https://github.com/inference-gateway/rust-sdk/compare/0.2.1...0.2.2) (2025-01-21)

### 👷 CI

* Enhance release workflow to output new version and publish conditionally ([424b92d](https://github.com/inference-gateway/rust-sdk/commit/424b92dc805d0fd507fb4465f1be9bd3b2af6054))

## [0.2.1](https://github.com/inference-gateway/rust-sdk/compare/0.2.0...0.2.1) (2025-01-21)

### 🐛 Bug Fixes

* Update README and lib.rs to include content in response and improve logging ([97e2a29](https://github.com/inference-gateway/rust-sdk/commit/97e2a2941e6912339ac4c8bfb7c24457728088d9))

## [0.2.0](https://github.com/inference-gateway/rust-sdk/compare/0.1.4...0.2.0) (2025-01-21)

### ✨ Features

* Add Taskfile for OpenAPI specification download and update devcontainer configuration ([7ea7473](https://github.com/inference-gateway/rust-sdk/commit/7ea7473e557405a3e856b77deaebe3096d30c613))
* Enhance content generation with structured messages and logging ([de4e8a7](https://github.com/inference-gateway/rust-sdk/commit/de4e8a74fb128f961854954785c462dd0db05bb6))

### 🔧 Miscellaneous

* Update OpenAPI specification from main repo ([c9c92f6](https://github.com/inference-gateway/rust-sdk/commit/c9c92f6b56afefa5de237752265f083c9be6c159))

## [0.1.4](https://github.com/inference-gateway/rust-sdk/compare/0.1.3...0.1.4) (2025-01-20)

### 🐛 Bug Fixes

* Correct launch.json path in devcontainer configuration ([86717c4](https://github.com/inference-gateway/rust-sdk/commit/86717c4f49c1b5b87cecfc33e19f62c5e93c58f0))

## [0.1.3](https://github.com/inference-gateway/rust-sdk/compare/0.1.2...0.1.3) (2025-01-20)

### ♻️ Improvements

* Update repository URLs in configuration and changelog ([586febf](https://github.com/inference-gateway/rust-sdk/commit/586febf9cddf7268e87153de45e3f6a72bcb851d))

## [0.1.2](https://github.com/inference-gateway/rust-sdk/compare/0.1.1...0.1.2) (2025-01-20)

### 👷 CI

- Enhance GitHub release configuration with asset management and custom release name ([da054cf](https://github.com/inference-gateway/rust-sdk/commit/da054cf30788d6179336307e4f957f6051ba5d44))

### 📚 Documentation

- Keep it simple ([17d7b79](https://github.com/inference-gateway/rust-sdk/commit/17d7b792201a3bfc735da913b60e3c23b7d0314c))

## [0.1.1](https://github.com/inference-gateway/rust-sdk/compare/0.1.0...0.1.1) (2025-01-20)

### 👷 CI

- Reintroduce publish job in release workflow for crates.io and reverse the order ([734875b](https://github.com/inference-gateway/rust-sdk/commit/734875bb64098c3e78be713d9da52a567a646482))
- Remove initialVersion from release configuration ([50ec13c](https://github.com/inference-gateway/rust-sdk/commit/50ec13c335ea895b52cb2a8c5942ed4998a0c15f))
- Simplify semantic-release command in release workflow ([c973164](https://github.com/inference-gateway/rust-sdk/commit/c97316421e5d05a0a2d6476169ba8f316bbff09e))
