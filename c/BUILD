load("@rules_cc//cc:defs.bzl", "cc_binary", "cc_import", "cc_library", "cc_test")
package(default_visibility = ["//visibility:public"])

cc_library(
    name = "cc_calculator",
    srcs = ["calculator.cc"],
    hdrs = ["calculator.h"],
    copts = ["-std=c++17", "-fexceptions"],  # Ensure exceptions are enabled
    linkopts = ["-lc++"],  # Explicitly link against C++ standard library
)