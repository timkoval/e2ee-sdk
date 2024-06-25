# e2ee-sdk

This project includes cargo wokspace that contains all the crates needed for End-to-End encryption usage on different platforms:
- `e2ee` crate provides Rust library for e2e encryption implemented with RustCrypto [`rsa`](https://github.com/RustCrypto/RSA) crate for symmetric encryption and [`AEADs`](https://github.com/RustCrypto/AEADs) for asymmetric encryption (TODO);
- `e2ee-cli` crate provides CLI interface for `rsa` and `aes-gcm` (TODO) **keys generation, file encryption and file decryption** with [`clap`](https://github.com/clap-rs/clap);
- `e2ee-sdk` crate provides cross-platform `uniffi` bindings for SDK functionality in Swift, Kotlin and Python;
- `uniffi-bindgen` - crate that provides cli tool for bindings generation with [`uniffi`](https://github.com/mozilla/uniffi-rs);

The bindings can be generated using `Justfile` ([`just`](https://github.com/casey/just) required) provided with the following commands:
- `just build-ios` - Swift bindings generation (XCode installation required)
- `just build-android` - Kotlin bindings generation (AndroidStudio installation required)
- `just build-python` - Python bindings generation

[Nix](https://nixos.org/) package manager and nix development environments are used for better reproducability. 
