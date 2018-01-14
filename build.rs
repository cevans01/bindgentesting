
extern crate cc;
extern crate bindgen;

use std::env;
use std::path::PathBuf;

pub fn main() {

    let target = env::var("TARGET").unwrap();

	// Compile cpp files into static library
	cc::Build::new()
        .cpp(true)
		.file("libfoo/foo.cpp")
		.compile("foo");


    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("libfoo/foo.hpp")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

}
