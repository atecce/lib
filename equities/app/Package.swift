// swift-tools-version:5.9
// An app for the equities dashboards, reading InfluxDB through the Rust
// `influx` crate (exported by uniffi in ../../Library). This package is the
// macOS app; Equities.xcodeproj builds the same sources for iOS.
//
//     cargo swift package -p macos@12 ios@15 -n Library -y   # from the workspace root
//     source ~/.profile && swift run                          # from here

import PackageDescription

let package = Package(
    name: "Equities",
    platforms: [.macOS(.v13)],
    dependencies: [.package(path: "../../Library")],
    targets: [
        .executableTarget(
            name: "Equities",
            dependencies: [.product(name: "Library", package: "Library")],
            // The Rust static library's HTTP client (reqwest) reads proxy
            // settings and uses the system's certificates.
            linkerSettings: [.linkedFramework("SystemConfiguration"), .linkedFramework("Security")]
        ),
    ]
)
