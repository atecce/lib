// swift-tools-version:5.5
// The swift-tools-version declares the minimum version of Swift required to build this package.
// Swift Package: Source

import PackageDescription;

let package = Package(
    name: "Source",
    platforms: [
        .macOS(.v10_15), .iOS(.v13)
    ],
    products: [
        .library(
            name: "Source",
            targets: ["Source"]
        )
    ],
    dependencies: [ ],
    targets: [
        .binaryTarget(name: "sourceFFI", path: "./sourceFFI.xcframework"),
        .target(
            name: "Source",
            dependencies: [
                .target(name: "sourceFFI")
            ]
        ),
    ]
)