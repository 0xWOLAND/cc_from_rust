use std::process::Command;
use std::path::PathBuf;

fn main() {
    // Run Bazel to build the C++ library
    let status = Command::new("bazel")
        .args(&["build", "//c:cc_calculator"])
        .status()
        .expect("Failed to build C++ library with Bazel");

    if !status.success() {
        panic!("Bazel build failed");
    }

    // Get the Bazel output directory
    let bazel_out_dir = PathBuf::from("../bazel-bin/c");

    // Inform Cargo where to find the library
    println!("cargo:rustc-link-search=native={}", bazel_out_dir.display());
    println!("cargo:rustc-link-lib=static=cc_calculator"); // Link C++ library
    println!("cargo:rustc-link-lib=c++");  // Explicitly link against libc++ for macOS
}
