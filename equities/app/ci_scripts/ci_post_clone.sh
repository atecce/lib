#!/bin/zsh
# Xcode Cloud runs this after cloning, before building Equities.xcodeproj.
#
# The app links the Rust code through ../../Library, a Swift package whose
# Swift sources are in git but whose compiled libraries (liblib.a, inside
# bibleFFI.xcframework) are not: build them here, as on the Mac (see
# equities/app/Package.swift), but for iOS only and optimized.
#
# Xcode Cloud's machines have network access and Homebrew, not Rust.
set -euo pipefail

# The workspace root: equities/app/ci_scripts -> ../../..
cd "${CI_PRIMARY_REPOSITORY_PATH:-${0:A:h:h:h:h}}"

# Rust, with the iPhone and Simulator targets.
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal
source "$HOME/.cargo/env"
rustup target add aarch64-apple-ios aarch64-apple-ios-sim x86_64-apple-ios

# cargo-swift, the version the Mac builds Library with.
cargo install cargo-swift --version 0.11.0 --locked

# Library for iOS, release build. .cargo/config.toml sets the deployment
# targets (IPHONEOS_DEPLOYMENT_TARGET=15.0) that keep aws-lc's C objects
# linkable.
cargo swift package -p ios@15 -n Library -y -r
