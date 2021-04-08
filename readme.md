# ccdb:  The cmd game engine  
(Thats also multi threaded)  
  
# How to use  
  
SOME FEATURES ARE ONLY TESTED ON WINDOWS THIS INCLUDES KEYIN AND CURSOR!(these are made in C)  
  
  
## Learn how to use it by example  

```rust

extern crate ccdb;
use ccdb::acore::{core,screen}; // there are 2 diffrent Acores there is Acore and there is banana both work the same way when talking to them 
use ccdb::loader::{load,toMap};// this is the loader which makes it so you can load a map from file or load a map from string 
use ccdb::keyin; // For key input
use ccdb::cursor; // for moving the cursor


pub fn main() {
    
    cursor::clear(); // clears the screen
    cursor::hideCursor(); // hides the cursor
    

    let x = Acore{
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
    
    let mut a = x.setup(); // set up the Acore struct 
    //! map loading stuff
    f.loadmap(load("./maps/map.rmap")); // loads in the map
    let player = f.findX("@".to_string()); // gets the player position in the screen.chars section findAllOfX works the same but returns a vector 
    f.loadmap(toMap("#####\n33333".to_string()));//if you want to make a map out of a string 

    //! physics stuff
    a.addphys(player); // adds phycis to an object 
    a.addphysForAllX(&f,"I".to_string());// adds physics to all objects with the correct char
    a.removephys(player); // remove physics form an object 
    a.changePhysics(-1); // change the gravity constant to make gravity stronger or weaker or upside down 

    loop {
        
        cursor::clear();// clear the screen
        a.render(&mut f);// renders it all does not work with Bcore
        println!("{}",a.render(&mut f););// if you are using Bcore the output gets output in a string ( Does not work with Acore )
        
      

    }

  
}
```  
## Differance between Bcore and Acore  
Core is mutli threaded and does not work very efficiently with big amount of text  
Bcore is the old algorithm for making the text  
Bcore also outputs into a string instead of into the console  
Their names are Acore: Banana core, Bcore: Olive core  
  
# Multi threading  
If you want to use multi threading you have to use Acore  
Using more than 2 threads is not really recomended  
The amount of threads cannot be odd  
If you have more threads than lines the program will crash  
  
# Contributing  
If you have linux it would be nice if you could test the keyin and cursor libraries to see if they will work on linux!  
  
# Roadmap  
v0.2.0: Add a function to find all of a certain character or just the first one  DONE  
v0.3.0: Being able to get the output in a string instead of the cmd   DONE  
v0.4.0: Physics and collision  DONE  
v0.5.0: Loading of ascii sprites from file  
v0.6.0: Key input rework  
v0.7.0: Rework of variable names and function names  
v0.8.0: No plans  
v0.9.0: No plans  
v1.0.0: No plans  

