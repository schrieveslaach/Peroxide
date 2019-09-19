use std::env;
use std::path::PathBuf;

macro_rules! feature(($name:expr) => (env::var(concat!("CARGO_FEATURE_", $name)).is_ok()));

fn main() {
    if feature!("OXIDIZE") {
        println!("cargo:rustc-link-lib=dylib=gfortran");
        println!("cargo:rustc-link-lib=dylib=openblas");

        let bindings = bindgen::Builder::default()
            .header("/usr/include/f77blas.h")
            .generate()
            .expect("Unable to generate openblas bindings");

        let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
        bindings
            .write_to_file(out_path.join("openblas.rs"))
            .expect("Couldn't write bindings!");
    }
}
