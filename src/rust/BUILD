load("@rules_rust//rust:defs.bzl", "rust_library", "rust_test")

rust_library(
    name = "matrix_ops_rust",
    srcs = ["src/lib.rs"],
    deps = ["//src/cpp:matrix_ops"],
)

rust_test(
    name = "matrix_ops_test",
    crate = ":matrix_ops_rust",
)
