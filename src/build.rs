//extern crate gcc;

fn main() {

    // compiles the c scripts
    cc::Build::new()
    .file("./src/c/key.c")
    .compile("key");
    
    cc::Build::new()
    .file("./src/c/cursorhand.c")
    .compile("cursor");
}