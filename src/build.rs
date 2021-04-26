//extern crate gcc;

fn main() {
    // compiles the c script
    cc::Build::new().file("./src/c/key.c").compile("key");

}
