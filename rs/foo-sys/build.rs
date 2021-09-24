use bindgen;
use cc;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=vendor/include/foo.h");
    println!("cargo:rerun-if-changed=vendor/src/foo.c");

    cc::Build::new()
        .file("vendor/src/foo.c")
        .include("vendor/include")
        .compile("foo");

    let bindings = bindgen::Builder::default()
        .header("vendor/include/foo.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    println!("cargo:rustc-link-lib=static=foo");
}
