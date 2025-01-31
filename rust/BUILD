load("@rules_rust//rust:defs.bzl", "rust_doc", "rust_library", "rust_test")

package(default_visibility = ["//visibility:public"])

rust_library(
    name = "rust_calculator",
    srcs = [
        "lib.rs",
        "ffi.rs",
    ],
    deps = [
        "//c:cc_calculator",
        "@crates//:libc",
    ],
)