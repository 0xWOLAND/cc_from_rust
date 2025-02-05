use std::process::Command;
use std::path::PathBuf;

fn main() {
    // 1. Build the C++ bridge code from Rust using cxx
    cxx_build::bridge("src/lib.rs")
        .flag_if_supported("-std=c++14")
        .include("../cc") // Ensure this is the correct path to matrix_lib headers
        .compile("cxx_bridge_demo");

    // 2. Invoke Bazel to build the C++ static library.
    let status = Command::new("bazel")
        .args(&["build", "//cc:matrix_lib"])
        .status()
        .expect("Failed to run bazel build");

    if !status.success() {
        panic!("Bazel build failed");
    }

    // 3. Locate the generated library and pass it to Cargo.
    let bazel_bin_lib = PathBuf::from("bazel-bin/cc");

    println!("cargo:rustc-link-search=native={}", bazel_bin_lib.display());
    println!("cargo:rustc-link-lib=static=matrix_lib");
    
    // Link the C++ standard library based on platform
    if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-lib=dylib=c++");
    } else if cfg!(target_os = "linux") {
        println!("cargo:rustc-link-lib=dylib=stdc++");
    } else if cfg!(target_os = "windows") {
        println!("cargo:rustc-link-lib=dylib=msvcrt");
    }
}
