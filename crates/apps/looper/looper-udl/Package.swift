// swift-tools-version:5.5.0

import PackageDescription

let package = Package(
        name: "LooperKit",
        products: [
            .library(
                    name: "LooperKit",
                    targets: ["LooperKit"]
            ),
        ],
        dependencies: [
        ],
        targets: [
            .systemLibrary(
                    name: "looperFFI"
            ),
            .target(
                    name: "LooperKit",
                    dependencies: ["looperFFI"]
            ),
        ]
)
