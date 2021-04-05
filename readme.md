# ccdb:  The cmd game engine  
(Thats also multi threaded)  

# How to use  

SOME FEATURES ARE ONLY TESTED ON WINDOWS THIS INCLUDES KEYIN AND CURSOR!(these are made in C)  


## Learn how to use it by example  

```rust

extern crate ccdb;
use ccdb::core::{core,screen}; // there are 2 diffrent cores there is Core and there is banana both work the same way when talking to them 
use ccdb::loader::load;// this is the loader which makes it so you can load a map from file
use ccdb::keyin; // For key input
use ccdb::cursor; // for moving the cursor


pub fn main() {
    
    cursor::clear(); // clears the screen
    cursor::hideCursor(); // hides the cursor
    

    let x = core{
        name: "Test project".to_string(), // name of the project 
        desc: " - A test project".to_string(), // descirption (short )
        linelenght: 20,// how many charecters per line
        lines: 4, // how many lines
        debug: true, // This will make it so that CCDB (VERSION) is not shown
        threads: 2, // How many threads 
        delay: 1,// delay between threads ( So that they print in the correct order)
    };
    
    
    let mut f = screen{
        chars: vec!("0".to_string(),"1".to_string()), // these are the different ascii "items" that get renderd X and Y are the cordinates 
        x: vec!(1,2),
        y: vec!(1,2),
        delay: 10,// delay between each frame for extra bonus use 100 ms
    };
    
    let mut a = x.setup(); // set up the core struct 
    f.loadmap(load("./maps/map.rmap")); // loads in the map

  
    
    loop {
        
        cursor::clear();// clear the screen
        a.render(&f);// renders it all
        
      

    }

  
}
```
## Differance between oldcore and core  
Core is mutli threaded and does not work very efficiently with big amount of text  
oldcore is the old algorithm for making the text   

# Multi threading  
If you want to use multi threading you have to use Core  
Using more than 2 threads is not really recomended  
The amount of threads cannot be odd  
If you have more threads than lines the program will crash  

# Contributing  
If you have linux it would be nice if you could test the keyin and cursor libraries to see if they will work on linux!  