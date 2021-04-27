//extern crate gcc;

fn main() {
    // compiles the c script
    println!("cargo:rerun-if-changed=src/c/key.c");
    cc::Build::new().file("./src/c/key.c").compile("key");
}
