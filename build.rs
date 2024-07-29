use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    // Get the output directory for the build artifacts
    let libname = "math_zig";
    let out_dir = dbg!(PathBuf::from(env::var("OUT_DIR").unwrap()));
    let zig_lib_out_dir = PathBuf::from("./zig/zig-out/lib");

    // Perform zib build
    let status = Command::new("zig")
        .current_dir("zig")
        .args(&["build"])
        .status()
        .expect("Failed to execute Zig compiler");

    if !status.success() {
        panic!("Zig compilation failed");
    }

    // Move the compiled library to the output directory
    let lib_filename = &format!("lib{}.so", libname);
    std::fs::rename(
        zig_lib_out_dir.join(lib_filename),
        out_dir.join(lib_filename),
    )
    .expect("Failed to move Zig library to output directory");

    // Tell Cargo where to find the library
    println!("cargo:rustc-link-search=native={}", out_dir.display());
    println!("cargo:rustc-link-lib=dylib={}", libname);

    // Set the rpath
    println!("cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN");
    println!("cargo:rustc-link-arg=-Wl,-rpath,{}", out_dir.display());
}
