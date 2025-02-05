load("@rules_cc//cc:defs.bzl", "cc_library")

cc_library(
    name = "matrix_lib",
    srcs = ["matrix.cc"],
    hdrs = ["matrix.h"],
    deps = [
        "@eigen//:eigen",
    ],
    # Force a static library so we can link it from Rust easily
    linkstatic = True,
    visibility = ["//visibility:public"],
)
