// use std::env;
use std::path::PathBuf;


// https://doc.rust-lang.org/cargo/reference/build-scripts.html


fn main() {
    // Tell cargo to look for shared libraries in the specified directory
    // println!("cargo:rustc-link-search=/usr/lib");

    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    // println!("cargo:rustc-link-lib=pcap");

    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo:rerun-if-changed=src/jankincai.c");

    cc::Build::new()
    .file("src/jankincai.c")
    .compile("jankincai");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("src/jankincai.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from("./src/");
    bindings
        .write_to_file(out_path.join("jankincai.rs"))
        .expect("Couldn't write bindings!");
}
