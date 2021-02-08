use bindgen::{Builder, CargoCallbacks};

use std::{env, path::PathBuf};

fn main() {
    // println!("cargo:rustc-link-lib=ntdll");

    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(CargoCallbacks))
        .opaque_type("_IMAGE_TLS_DIRECTORY64")
        .opaque_type("_KUSER_SHARED_DATA")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}