# Changelog

All notable changes to this project will be documented in this file.

## [0.9.1-rc.1](https://github.com/inference-gateway/rust-sdk/compare/0.9.0...0.9.1-rc.1) (2025-03-20)

### ♻️ Improvements

* Add streaming response structures for chat completion ([0dd7b56](https://github.com/inference-gateway/rust-sdk/commit/0dd7b5693f14d086acdacc7356b34caf73eb7035))
* Download the OpenAPI spec ([7701059](https://github.com/inference-gateway/rust-sdk/commit/7701059e64ae6073e73803b1ddb4fbc93dc35417))
* Remove Google provider from the Provider enum ([5a98051](https://github.com/inference-gateway/rust-sdk/commit/5a98051437c3cc6578b7de86ef937ae4736b7184))
* Remove nullable fields from OpenAPI schema definitions ([461750f](https://github.com/inference-gateway/rust-sdk/commit/461750facb43b70a3112f867cb689711804ca560))
* Rename default client method to new_default for clarity ([8735999](https://github.com/inference-gateway/rust-sdk/commit/87359996264e6d50a3ef078080794d3254ed9e06))
* Rename GenerateRequest to CreateChatCompletionRequest and remove ssevents field ([52c4008](https://github.com/inference-gateway/rust-sdk/commit/52c40089ec6c59fce0c9748f8f1c985cd81e6fab))
* Restructure response to be compatible with OpenAI ([88da800](https://github.com/inference-gateway/rust-sdk/commit/88da80062a23a272be04c54b2d81a667004ca066))
* Start by implementing default and base_url ([1afdaac](https://github.com/inference-gateway/rust-sdk/commit/1afdaac83a7cccd8405db1eeddb387dc0822260e))
* Update JSON response format to include new fields for chat completion ([38869a1](https://github.com/inference-gateway/rust-sdk/commit/38869a19293385f5c6660adb5c3b79afddf2b906))
* Update JSON response structure with new fields for chat completion ([340c02c](https://github.com/inference-gateway/rust-sdk/commit/340c02c006aa397b0ce8e7b512b9bf9c6e244f8a))
* Update tool call structures for chat completion and enhance argument parsing ([c867978](https://github.com/inference-gateway/rust-sdk/commit/c86797846676a266f477d906f1e9a9a2e12f41cd))

### 🐛 Bug Fixes

* Add output logging for version determination error in release workflow ([d49ba50](https://github.com/inference-gateway/rust-sdk/commit/d49ba50bd4111b28b722ae78657c6660909c0932))

## [0.9.0](https://github.com/inference-gateway/rust-sdk/compare/0.8.0...0.9.0) (2025-02-11)

### ✨ Features

* Add max_tokens option to generate_content ([#5](https://github.com/inference-gateway/rust-sdk/issues/5)) ([fc21cf2](https://github.com/inference-gateway/rust-sdk/commit/fc21cf234e694c8d27dd54bfdb169157cce21288))

## [0.8.0](https://github.com/inference-gateway/rust-sdk/compare/0.7.0...0.8.0) (2025-02-10)

### ✨ Features

* Tool-Use functions call ([#4](https://github.com/inference-gateway/rust-sdk/issues/4)) ([efd8e6b](https://github.com/inference-gateway/rust-sdk/commit/efd8e6b9c4d9bf99f776e3f26a8a627713757482)), closes [#3](https://github.com/inference-gateway/rust-sdk/issues/3)

### ♻️ Improvements

* Remove tool_calls from Message struct and update related tests ([265bdec](https://github.com/inference-gateway/rust-sdk/commit/265bdec49b9a4394cb78e0bf286f0c267a9a3b06))

### 👷 CI

* Improve the Release Process Allowing to have Release Candidates ([#3](https://github.com/inference-gateway/rust-sdk/issues/3)) ([b97e5da](https://github.com/inference-gateway/rust-sdk/commit/b97e5da228ccfa8719f0211a0a28fa344bf4954c))

### 📚 Documentation

* Remove unnecessary code block from README.md ([dcb7c26](https://github.com/inference-gateway/rust-sdk/commit/dcb7c26493be82790615b7434acbf3ddb13b4fc1))

## [0.8.0-rc.2](https://github.com/inference-gateway/rust-sdk/compare/0.8.0-rc.1...0.8.0-rc.2) (2025-02-10)

### ♻️ Improvements

* Implement Default trait for Message and MessageRole structs ([cb43ca8](https://github.com/inference-gateway/rust-sdk/commit/cb43ca8034cec51bb4037e21dd1be9a8ce7312f3))
* Initialize the client with tools optionally ([9c56170](https://github.com/inference-gateway/rust-sdk/commit/9c56170a17939b7ecf8a71ae04425313cf97727c))
* Use in the tests two different conventions to see what's possible ([eef16e8](https://github.com/inference-gateway/rust-sdk/commit/eef16e8afedd2b28a2a54aa25f2364adfe11973b))

### 📚 Documentation

* Enhance InferenceGatewayAPI docblocks with additional error handling and return details ([b666c33](https://github.com/inference-gateway/rust-sdk/commit/b666c3338d81e193396d43d927a5b09569f70752))
* Improve the docs for Tool-Use, update it ([9e5f5da](https://github.com/inference-gateway/rust-sdk/commit/9e5f5da14120b553ef045502dd704be5dcccd67f))
* Improve the example by mentioning the workflow of messages exchange ([ef474b4](https://github.com/inference-gateway/rust-sdk/commit/ef474b427fd28527b589d7ff966acda4ea1d3cec))
* Remove redundant whitespace in InferenceGatewayAPI documentation ([422e857](https://github.com/inference-gateway/rust-sdk/commit/422e8571c862547bf78aa2415528bf0814084319))
* Resort functions examples, healthcheck is less important so put it at the bottom ([e6eb55d](https://github.com/inference-gateway/rust-sdk/commit/e6eb55d89a32a39603496303ed2b4d05b8bbf885))
* Update the example in the README.md ([0fda0dc](https://github.com/inference-gateway/rust-sdk/commit/0fda0dc4dbed741dbfe8b62197ed6aea43e6b9e5))

### ✅ Miscellaneous

* Add assertion for tool_calls length in response test ([4edcd4b](https://github.com/inference-gateway/rust-sdk/commit/4edcd4b43e64bca6a1ac1856e8e57c4eabd0e0a8))

## [0.8.0-rc.1](https://github.com/inference-gateway/rust-sdk/compare/0.7.1-rc.3...0.8.0-rc.1) (2025-02-09)

### ✨ Features

* Add optional tool_call_id to Message struct and update serialization tests ([d51e911](https://github.com/inference-gateway/rust-sdk/commit/d51e9115bbcd4fb5f098e99dfda9f11c00febf07))

## [0.7.1-rc.3](https://github.com/inference-gateway/rust-sdk/compare/0.7.1-rc.2...0.7.1-rc.3) (2025-02-09)

### ♻️ Improvements

* Remove ToolParameterType and ToolParameter structs to simplify code ([0e4d1b4](https://github.com/inference-gateway/rust-sdk/commit/0e4d1b4a0519adb1c23dc5f25f841af72e7c6527))
* Rename parameters to arguments in ToolFunctionResponse and update ToolCallResponse structure ([3f36c5f](https://github.com/inference-gateway/rust-sdk/commit/3f36c5f969a7b46417613c9fd11f1760d841ff76))

## [0.7.1-rc.2](https://github.com/inference-gateway/rust-sdk/compare/0.7.1-rc.1...0.7.1-rc.2) (2025-02-09)

### 🐛 Bug Fixes

* Enhance Tool and ToolFunction structures with description and parameters updates, use JSON raw value for now ([cdf6eca](https://github.com/inference-gateway/rust-sdk/commit/cdf6eca712c07003841cea55d57ae62cfa3bca2b))

## [0.7.1-rc.1](https://github.com/inference-gateway/rust-sdk/compare/0.7.0...0.7.1-rc.1) (2025-02-08)

### ♻️ Improvements

* Remove tool_calls from Message struct and update related tests ([265bdec](https://github.com/inference-gateway/rust-sdk/commit/265bdec49b9a4394cb78e0bf286f0c267a9a3b06))

### 👷 CI

* Improve the Release Process Allowing to have Release Candidates ([#3](https://github.com/inference-gateway/rust-sdk/issues/3)) ([b97e5da](https://github.com/inference-gateway/rust-sdk/commit/b97e5da228ccfa8719f0211a0a28fa344bf4954c))

### 📚 Documentation

* Remove unnecessary code block from README.md ([dcb7c26](https://github.com/inference-gateway/rust-sdk/commit/dcb7c26493be82790615b7434acbf3ddb13b4fc1))

## [0.7.0](https://github.com/inference-gateway/rust-sdk/compare/0.6.3...0.7.0) (2025-02-07)

### ✨ Features

* Add tool call functionality to Message struct and related types ([dba7f19](https://github.com/inference-gateway/rust-sdk/commit/dba7f196fb048c55c8e6fdeb0fee22a99a8c9ec0))
* Implement Tool-Use ([#2](https://github.com/inference-gateway/rust-sdk/issues/2)) ([1a484a7](https://github.com/inference-gateway/rust-sdk/commit/1a484a724748a371db4cffa8d94f2668b1d5b96b))

### 📚 Documentation

* **openapi:** Add Tool schema and update Message structure in OpenAPI definition ([ae377e9](https://github.com/inference-gateway/rust-sdk/commit/ae377e9d43ae117852b487533762b5449b5bf2ca))

## [0.6.3](https://github.com/inference-gateway/rust-sdk/compare/0.6.2...0.6.3) (2025-02-04)

### 🔧 Miscellaneous

* Add Clone trait to Message struct for improved usability ([e5ebd50](https://github.com/inference-gateway/rust-sdk/commit/e5ebd50874138a8a5a7cc30a154929bca4244929))

## [0.6.2](https://github.com/inference-gateway/rust-sdk/compare/0.6.1...0.6.2) (2025-02-02)

### 📚 Documentation

* Update README to include serde dependencies for stream response content deserialization ([b9d2fa7](https://github.com/inference-gateway/rust-sdk/commit/b9d2fa7e28e089d57b83a6f7baf7aae8087bf224))

## [0.6.1](https://github.com/inference-gateway/rust-sdk/compare/0.6.0...0.6.1) (2025-02-02)

### 📚 Documentation

* Add Streaming Content section to README with usage examples ([4e135a4](https://github.com/inference-gateway/rust-sdk/commit/4e135a43d779d30424148bf4c4b80c65b7af7fdf))

## [0.6.0](https://github.com/inference-gateway/rust-sdk/compare/0.5.8...0.6.0) (2025-02-02)

### ✨ Features

* Add streaming support for content generation and enhance error handling ([5c574c3](https://github.com/inference-gateway/rust-sdk/commit/5c574c313a6839d538bc65de28fb5264f3e9e006))
* Implement streaming API so the client can receive Server Sent Events ([32060e0](https://github.com/inference-gateway/rust-sdk/commit/32060e0c30c68211a9cea5c3c34f53f382ec15f5))

### ♻️ Improvements

* Refactor content generation stream to use SSEvents struct for improved event handling ([1411d15](https://github.com/inference-gateway/rust-sdk/commit/1411d15fce253aaed53fffa97f145e947c1f435c))

### 🔧 Miscellaneous

* Add TODO to enhance the error handling of streaming with retries ([4162ad7](https://github.com/inference-gateway/rust-sdk/commit/4162ad7aada9ed86295b330c9af7fdb3b8d2d8a6))
* Remove unnecessary blank line in content generation stream ([b4698b8](https://github.com/inference-gateway/rust-sdk/commit/b4698b8930b9d1570e2565e562027b0b431f5dbb))
* Update openapi - removed google AI Studio and Implemented Streaming ([924e23e](https://github.com/inference-gateway/rust-sdk/commit/924e23eb6214f417549db30822aca40f56ba68d8))

## [0.5.8](https://github.com/inference-gateway/rust-sdk/compare/0.5.7...0.5.8) (2025-01-29)

### 📚 Documentation

* Update health check method to use async/await for improved API responsiveness ([0e5d245](https://github.com/inference-gateway/rust-sdk/commit/0e5d2453d5cc8ff06f9e2940902948334b2acde1))
* Update README with client usage examples and enhance API interaction details ([4d86541](https://github.com/inference-gateway/rust-sdk/commit/4d865419c9576173ff30c9b77123fe2976e80abd))

## [0.5.7](https://github.com/inference-gateway/rust-sdk/compare/0.5.6...0.5.7) (2025-01-29)

### 🔧 Miscellaneous

* Add case-insensitive provider aliases and update URL generation ([0715d11](https://github.com/inference-gateway/rust-sdk/commit/0715d113a2a5ffbf243f8bb2fae331a8c115228a))

## [0.5.6](https://github.com/inference-gateway/rust-sdk/compare/0.5.5...0.5.6) (2025-01-29)

### ✅ Miscellaneous

* Remove unnecessary lowercase conversion for provider string and enhance error handling in API responses ([cdeb27c](https://github.com/inference-gateway/rust-sdk/commit/cdeb27caebe9fda6c5b3165492eae939be23a581))

## [0.5.5](https://github.com/inference-gateway/rust-sdk/compare/0.5.4...0.5.5) (2025-01-29)

### 🐛 Bug Fixes

* Ensure provider string is lowercase in URL generation for consistency ([f84314c](https://github.com/inference-gateway/rust-sdk/commit/f84314c76286ddfa46b01b269f07b039a17c064d))

## [0.5.4](https://github.com/inference-gateway/rust-sdk/compare/0.5.3...0.5.4) (2025-01-29)

### 📚 Documentation

* Update main function to return GatewayError instead of Box<dyn Error> ([9c8bbc3](https://github.com/inference-gateway/rust-sdk/commit/9c8bbc3d7b341184568aea5f06fe22ee1cf441a2))

### 🔧 Miscellaneous

* Implement TryFrom for Provider just in case users want to convert the string to an enum ([3da7e50](https://github.com/inference-gateway/rust-sdk/commit/3da7e50619fbd8dc0ae803b52ec935d07251cacd))
* Update dependencies and refactor main function for async support ([b99d520](https://github.com/inference-gateway/rust-sdk/commit/b99d5205a8457cc99b37f4a8adf20e46604d3b53))

## [0.5.3](https://github.com/inference-gateway/rust-sdk/compare/0.5.2...0.5.3) (2025-01-28)

### 📚 Documentation

* Update README to include InferenceGatewayAPI in import statements for clarity ([b6aba41](https://github.com/inference-gateway/rust-sdk/commit/b6aba41fd1beee192bf746a32de4b281b247d8fa))

### 🔧 Miscellaneous

* Update error handling to use thiserror crate and improve GatewayError structure ([fe85dcb](https://github.com/inference-gateway/rust-sdk/commit/fe85dcbfacdf5737f9c03ec23341a7254202e18f))

### ✅ Miscellaneous

* Add unit tests for GatewayError handling in InferenceGatewayClient ([6b7e6d4](https://github.com/inference-gateway/rust-sdk/commit/6b7e6d49494c08884e3dd93ff50406eda3e8cae5))

## [0.5.2](https://github.com/inference-gateway/rust-sdk/compare/0.5.1...0.5.2) (2025-01-28)

### 🐛 Bug Fixes

* Update model structure to use 'name' instead of 'id' for consistency ([3e0ea86](https://github.com/inference-gateway/rust-sdk/commit/3e0ea86fb9f9daedb6f256c149adf0f7fbf27349))

### 📚 Documentation

* **fix:** Update README to log model 'name' instead of 'id' for consistency ([4dba8fe](https://github.com/inference-gateway/rust-sdk/commit/4dba8fe7391226d3b0b33a70393b86f00a4c867d))

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
