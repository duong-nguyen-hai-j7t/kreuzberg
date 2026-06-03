// swift-tools-version: 6.0
import PackageDescription

let package = Package(
  name: "Kreuzberg",
  platforms: [
    .macOS(.v13),
    .iOS(.v16),
  ],
  products: [
    .library(name: "Kreuzberg", targets: ["Kreuzberg"])
  ],
  targets: [
    .target(
      name: "RustBridgeC",
      path: "packages/swift/Sources/RustBridgeC",
      publicHeadersPath: "."
    ),
    .target(
      name: "RustBridge",
      dependencies: ["RustBridgeC"],
      path: "packages/swift/Sources/RustBridge",
      linkerSettings: [
        .unsafeFlags([
          "-Ltarget/release",
          "-Ltarget/debug",
        ]),
        .linkedLibrary("kreuzberg_swift"),
        .linkedFramework("Security", .when(platforms: [.macOS, .iOS])),
        .linkedFramework("CoreFoundation", .when(platforms: [.macOS, .iOS])),
        .linkedFramework("SystemConfiguration", .when(platforms: [.macOS])),
      ]
    ),
    .target(
      name: "Kreuzberg",
      dependencies: ["RustBridge"],
      path: "packages/swift/Sources/Kreuzberg"
    ),
  ]
)
