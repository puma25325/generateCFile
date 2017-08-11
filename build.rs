use std::env;
use std::path::Path;

fn main(){
    let src = Path::new("example.c");
    let out_dir = env::var("OUT_DIR").unwrap();
    cc::Build::new()
        .file(src)
        .compile("example");
    print!("cargo:rerun-if-changed={}", src.display());
}