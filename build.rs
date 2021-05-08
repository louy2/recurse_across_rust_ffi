// Cargo build script to build C
fn main() {
    // Tell Cargo that if src/hello.c changes, to rerun this build script.
    println!("cargo:rerun-if-changed=src/into_rust.c");
    // Use the `cc` crate to build a C file and statically link it.
    cc::Build::new().file("src/into_rust.c").compile("into_rust");
}
