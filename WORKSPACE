load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")
load("@bazel_tools//tools/build_defs/repo:git.bzl", "git_repository", "new_git_repository")

# To find additional information on this release or newer ones visit:
# https://github.com/bazelbuild/rules_rust/releases
http_archive(
    name = "rules_rust",
    integrity = "sha256-Weev1uz2QztBlDA88JX6A1N72SucD1V8lBsaliM0TTg=",
    urls = ["https://github.com/bazelbuild/rules_rust/releases/download/0.48.0/rules_rust-v0.48.0.tar.gz"],
)

load("@rules_rust//rust:repositories.bzl", "rules_rust_dependencies", "rust_register_toolchains")

rules_rust_dependencies()
rust_register_toolchains()

new_git_repository(
    name = "libc",
    build_file = "libc.BUILD",
    remote = "https://github.com/rust-lang/libc",
    commit = "6ec4f81a3852797410b80296d3afd61f2b255a36",
    shallow_since = "1484672371 +0000"
)
