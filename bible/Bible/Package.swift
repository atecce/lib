// swift-tools-version:5.5
// The swift-tools-version declares the minimum version of Swift required to build this package.
// Swift Package: Bible

import PackageDescription;

let package = Package(
    name: "Bible",
    platforms: [
        .macOS(.v10_15), .iOS(.v13)
    ],
    products: [
        .library(
            name: "Bible",
            targets: ["Bible"]
        )
    ],
    dependencies: [ ],
    targets: [
        .binaryTarget(name: "bibleFFI", path: "./bibleFFI.xcframework"),
        .target(
            name: "Bible",
            dependencies: [
                .target(name: "bibleFFI")
            ]
        ),
    ]
)