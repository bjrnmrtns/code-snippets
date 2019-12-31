extern crate cmake;
extern crate bindgen;

use cmake::Config;
use std::env;
use std::path::PathBuf;

fn main()
{
    let dst = Config::new("borc").build();
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=bor");

    println!("cargo:rerun-if-changed=borc/include/bor.h");
    let bindings = bindgen::Builder::default()
        .header("borc/include/bor.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out_path.join("bindings.rs"))
            .expect("Couldn't write bindings!");
}
