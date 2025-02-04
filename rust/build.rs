fn main() {
    cxx_build::bridge("src/bridge.rs")
        .file("../bazel/calculator.cc")
        .include("../bazel")
        .flag_if_supported("-std=c++17")
        .compile("calculator");

    println!("cargo:rerun-if-changed=src/bridge.rs");
    println!("cargo:rerun-if-changed=../bazel/calculator.cc");
}
