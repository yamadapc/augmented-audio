// swift-tools-version:5.5
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
    name: "SequencerUI",
    platforms: [
        .macOS(.v10_15),
        .iOS(.v15),
    ],
    products: [
        // Products define the executables and libraries a package produces, and make them visible to other packages.
        .library(
            name: "SequencerUI",
            targets: ["SequencerUI"]
        ),
    ],
    dependencies: [
        // Dependencies declare other packages that this package depends on.
        // .package(url: /* package url */, from: "1.0.0"),
        .package(url: "https://github.com/sammysmallman/OSCKit", from: "3.1.0"),
        .package(url: "https://github.com/apple/swift-log.git", from: "1.0.0"),
        .package(url: "https://github.com/sindresorhus/KeyboardShortcuts", from: "1.4.0"),
        .package(url: "https://github.com/nalexn/ViewInspector", from: "0.9.1"),
        .package(url: "https://github.com/nicklockwood/LRUCache.git", .upToNextMinor(from: "1.0.0")),
        .package(url: "https://github.com/apple/swift-collections.git", .upToNextMajor(from: "1.0.0")),
    ],
    targets: [
        // Targets are the basic building blocks of a package. A target can define a module or a test suite.
        // Targets can depend on other targets in this package, and on products in packages this package depends on.
        .target(
            name: "SequencerUI",
            dependencies: [
                .productItem(name: "OSCKit", package: "OSCKit", condition: nil),
                .product(name: "Logging", package: "swift-log"),
                .product(name: "KeyboardShortcuts", package: "KeyboardShortcuts", condition: .when(platforms: [.macOS])),
                .product(name: "LRUCache", package: "LRUCache"),
                .product(name: "Collections", package: "swift-collections"),
            ]
        ),
        .testTarget(
            name: "SequencerUITests",
            dependencies: [
                "SequencerUI",
                "ViewInspector",
            ]
        ),
    ]
)