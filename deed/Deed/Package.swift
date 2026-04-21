// swift-tools-version:5.5
// The swift-tools-version declares the minimum version of Swift required to build this package.
// Swift Package: Deed

import PackageDescription;

let package = Package(
    name: "Deed",
    platforms: [
        .macOS(.v10_15), .iOS(.v13)
    ],
    products: [
        .library(
            name: "Deed",
            targets: ["Deed"]
        )
    ],
    dependencies: [ ],
    targets: [
        .binaryTarget(name: "sourceFFI", path: "./sourceFFI.xcframework"),
        .target(
            name: "Deed",
            dependencies: [
                .target(name: "sourceFFI")
            ]
        ),
    ]
)