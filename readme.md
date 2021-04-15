# ccdb:  The cmd game engine  
(Thats also multi threaded)  
  
# How to use  
  
SOME FEATURES ARE ONLY TESTED ON WINDOWS THIS INCLUDES KEYIN AND CURSOR!(these are made in C)  
  
  
## Learn how to use it by example  
  
  
```rust  
extern crate ccdb;
use ccdb::acore::{core,screen}; // there are 2 diffrent cores there is Core and there is banana both work the same way when talking to them 
use ccdb::loader::{load,toMap};// this is the loader which makes it so you can load a map from file or load a map from string 
use ccdb::loader;
use ccdb::keyin; // For key input
use ccdb::keycode; // For key input
use ccdb::cursor; // for moving the cursor
use ccdb::sprite; // for sprites
use ccdb::collision; // for collision




pub fn main() {
    
    cursor::clear(); // clears the screen
    cursor::hideCursor(); // hides the cursor
    let assets = loader::loadFromFolder("./maps/".to_string(),".rmap".to_string(),".rsprite".to_string()); // get a folder of maps and sprites 
    let map1 = assets.loadAssetMap("map");//load an assets from the folder struct 
    let psprite = assets.loadAssetSprite("sprite");//


    let x = core{// set up the core
        name: "Test project".to_string(),//name 
        desc: " - A test project".to_string(),// descirption
        linelenght: 40,// ammount of chars per line
        lines: 8,//amount of lines
        debug: true,//debug 
        threads: 4,// ammount of threads
        output_string: false,// if you want the output in string form or it just to printed out to the console directly 
    };
    
    
    let mut f = screen{// screen is where everything will be stored and how it should be renderd 
        chars: vec!("0".to_string(),"1".to_string()),
        x: vec!(1,2),
        y: vec!(1,2),
        delay: 10,// delay between each frame
        sprite: Vec::new(),// this is where all sprites go
    };
    
    let mut a = x.setup();// setup the core
    f.loadmap(map1);//load a map from a map struct 
    f.loadmap(toMap("#####\n33333".to_string()));//if you want to make a map out of a string 
    f.loadmap(load("./maps/map.rmap"));
    let run = true;
    let player = f.findX("@".to_string());// find a chars position
    
    a.addphysics(player);// add physics to char position
    a.addphysicsForAllX(&f,"I".to_string());
    a.removephysics(player);//remove physics for char pos
    a.changePhysics(1);// change the phycis 
    let mut sprite = psprite;// sprite loaded from assets otherwise its sprite::load("./maps/sprite.rsprite");
    f.sprite.push(sprite);//add the sprite to the sprite rendering list 
    f.sprite[0].setxy(2,-2);// set the sprites position
    f.sprite[0].movexy(5, 0);// move the sprite
    
    let mut dir = 0;
    loop {
        
        cursor::clear();
        cursor::hideCursor();
        //a.render(&mut f);
        println!("{}",a.render(&mut f));//render the entire screen
        f.sprite[0].movexy(1, 0);
        
        //standard key input system
        if keyin::keydown(){

        
            match keyin::getkey(){
                keycode::A =>{
                    f.x[player as usize] -= 1;
                    if collision::getcollision(player as usize, &f.gmap())//how to get the collision must pass in the screen
                    {
                        f.x[player as usize] += 1;
                    }
                }
                keycode::D =>{
                    f.x[player as usize] += 1;
                    if collision::getcollision(player as usize, &f.gmap())
                    {
                        f.x[player as usize] -= 1;
                    }
                }
                keycode::W =>{
                    f.y[player as usize] -= 1;
                    if collision::getcollision(player as usize, &f.gmap())
                    {
                        f.y[player as usize] += 1;
                    }
                }
                keycode::S =>{
                    f.y[player as usize] += 1;
                    if collision::getcollision(player as usize, &f.gmap())
                    {
                        f.y[player as usize] -= 1;
                    }
                }
                keycode::SPACE =>{
                    for x in 0..10{
                        f.x[player as usize] += 1;
                        if collision::getcollision(player as usize, &f.gmap())
                        {
                            f.x[player as usize] -= 1;
                            break
                        }
                    }   
                }
                _ =>{

                }
        }   
        }
      

    }

  
}


```  
  
## Differance between Bcore and Acore  
Core is mutli threaded and does not work very efficiently with big amount of text  
Bcore is the old algorithm for making the text  
Their names are Acore: Banana core, Bcore: Olive core  
Bcore is faster than Acore but Bcore is just weird  
  
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
v0.5.0: Loading of ascii sprites from file  DONE  
v0.6.0: Key input rework  DONE (+ some rework to the acore(IT works allot better))  
v0.7.0: Rework of variable names and function names  DONE  
v0.8.0: Adding a way to load in a folder  DONE  
v0.9.0: Getting the code ready for 1.0.0  DONE  
v1.0.0: optimization    