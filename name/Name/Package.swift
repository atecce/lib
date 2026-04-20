// swift-tools-version:5.5
// The swift-tools-version declares the minimum version of Swift required to build this package.
// Swift Package: Name

import PackageDescription;

let package = Package(
    name: "Name",
    platforms: [
        .macOS(.v10_15), .iOS(.v13)
    ],
    products: [
        .library(
            name: "Name",
            targets: ["Name"]
        )
    ],
    dependencies: [ ],
    targets: [
        .binaryTarget(name: "nameFFI", path: "./nameFFI.xcframework"),
        .target(
            name: "Name",
            dependencies: [
                .target(name: "nameFFI")
            ]
        ),
    ]
)