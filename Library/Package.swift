// swift-tools-version:5.5
// The swift-tools-version declares the minimum version of Swift required to build this package.
// Swift Package: Library

import PackageDescription;

let package = Package(
    name: "Library",
    platforms: [
        .macOS(.v12), .iOS(.v15)
    ],
    products: [
        .library(
            name: "Library",
            targets: ["Library"]
        )
    ],
    dependencies: [ ],
    targets: [
        .binaryTarget(name: "bibleFFI", path: "./bibleFFI.xcframework"),
        .target(
            name: "Library",
            dependencies: [
                .target(name: "bibleFFI")
            ]
        ),
    ]
)