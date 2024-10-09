// build.rs

use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    println!(
        // "cargo:rustc-link-arg=/LIBPATH:C:\\Users\\aubte\\source\\repos\\softcam\\dist\\lib\\x64\\"
        "cargo:rustc-link-arg=/LIBPATH:C:\\Users\\aubte\\source\\repos\\softcam\\dist\\lib\\x64\\"
    );
    println!("cargo:rustc-link-arg=/STACK:300000000");
    println!("cargo:rustc-link-lib=softcam");
    println!("cargo::rerun-if-changed=build.rs");
}
