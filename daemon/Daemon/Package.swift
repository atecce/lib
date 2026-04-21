// swift-tools-version:5.5
// The swift-tools-version declares the minimum version of Swift required to build this package.
// Swift Package: Daemon

import PackageDescription;

let package = Package(
    name: "Daemon",
    platforms: [
        .macOS(.v10_15), .iOS(.v13)
    ],
    products: [
        .library(
            name: "Daemon",
            targets: ["Daemon"]
        )
    ],
    dependencies: [ ],
    targets: [
        .binaryTarget(name: "sourceFFI", path: "./sourceFFI.xcframework"),
        .target(
            name: "Daemon",
            dependencies: [
                .target(name: "sourceFFI")
            ]
        ),
    ]
)