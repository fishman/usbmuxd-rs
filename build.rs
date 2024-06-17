use std::path::PathBuf;
use std::env;

fn main() {
    // Paths to the header files
    let headers = vec![
        "/usr/include/usbmuxd.h",
        "/usr/include/usbmuxd-proto.h",
    ];

    for header in &headers {
        // Tell Cargo to invalidate the built crate whenever the headers change
        println!("cargo:rerun-if-changed={}", header);

        // Generate bindings
        let bindings = bindgen::Builder::default()
            .header(header.to_string())
            .generate()
            .expect("Unable to generate bindings");

        // Write the bindings to the $OUT_DIR/bindings.rs file
        let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
        let output_file = format!("bindings_{}.rs", header.replace("/", "_"));
        println!("{}", output_file);
        bindings
            .write_to_file(out_path.join(output_file))
            .expect("Couldn't write bindings!");
    }
}
