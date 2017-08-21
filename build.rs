use std::env;
use std::path::PathBuf;

fn main(){
    let src_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("bindC")
        .join("src");
    let include_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("bindC")
        .join("include");
    cc::Build::new()
        .file(src_dir.join("example.c"))
        .include(&include_dir)
        .compile("example");
    let header_path = include_dir.join("example.h");
    let include_dir_str = include_dir.to_str().unwrap().to_owned();
    let bindings = bindgen::Builder::default()
        .header(header_path.to_str().unwrap())
        .clang_arg("-I")
        .clang_arg(&include_dir_str)
        .generate()
        .expect("Unable to generate bindings");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    println!("cargo:rustc-link-search=native={}", out_path.display());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
    println!("cargo:rerun-if-changed=bindC/src/example.c");
    println!("cargo:rerun-if-changed=bindC/include/example.h");
}