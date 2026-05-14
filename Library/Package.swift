// swift-tools-version:5.5
// The swift-tools-version declares the minimum version of Swift required to build this package.
// Swift Package: Library

import PackageDescription;

let package = Package(
    name: "Library",
    platforms: [
        .macOS(.v10_15), .iOS(.v13)
    ],
    products: [
        .library(
            name: "Library",
            targets: ["Library"]
        )
    ],
    dependencies: [ ],
    targets: [
        .binaryTarget(name: "libFFI", path: "./libFFI.xcframework"),
        .target(
            name: "Library",
            dependencies: [
                .target(name: "libFFI")
            ]
        ),
    ]
)
