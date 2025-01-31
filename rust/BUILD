load("@rules_rust//rust:defs.bzl", "rust_binary", "rust_doc", "rust_library", "rust_test")

package(default_visibility = ["//visibility:public"])

rust_library(
    name = "rust_calculator",
    srcs = ["src/lib.rs"],
    deps = ["//c:calculator"],
    edition = "2021",
    rustc_flags = [
        "-L", "$(BINDIR)/c",  # Correct path to the library directory
    ],
)