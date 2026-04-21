// swift-tools-version:5.5
// The swift-tools-version declares the minimum version of Swift required to build this package.
// Swift Package: Book

import PackageDescription;

let package = Package(
    name: "Book",
    platforms: [
        .macOS(.v10_15), .iOS(.v13)
    ],
    products: [
        .library(
            name: "Book",
            targets: ["Book"]
        )
    ],
    dependencies: [ ],
    targets: [
        .binaryTarget(name: "nameFFI", path: "./nameFFI.xcframework"),
        .target(
            name: "Book",
            dependencies: [
                .target(name: "nameFFI")
            ]
        ),
    ]
)