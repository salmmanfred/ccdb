# THE OLD TUTORIAL  
  
```rust

extern crate ccdb;
use ccdb::acore::{core,screen}; // there are 2 diffrent Acores there is Acore and there is banana both work the same way when talking to them 
use ccdb::loader::{load,toMap};// this is the loader which makes it so you can load a map from file or load a map from string 
use ccdb::keyin; // For key input
use ccdb::cursor; // for moving the cursor
use ccdb::keycode; // for key codes (work in progress)
use ccdb::sprite; // for sprites
use ccdb::collision; // for collision

pub fn main() {
    
    cursor::clear(); // clears the screen
    cursor::hideCursor(); // hides the cursor
    let assets = loader::loadFromFolder("./maps/".to_string(),".rmap".to_string(),".rsprite".to_string());//you can load assets using loadFromFolder you will be rewarded with a folder struct 
    let map1 = assets.loadAssetMap("map");// you can get your map using assets.loadAssetMap for getting the map
    let psprite = assets.loadAssetSprite("sprite");// sprite can be found using this 

    let x = Acore{
        name: "Test project".to_string(), // name of the project 
        desc: " - A test project".to_string(), // descirption (short )
        linelenght: 20,// how many charecters per line
        lines: 4, // how many lines
        debug: true, // This will make it so that CCDB (VERSION) is not shown
        threads: 2, // How many threads 
        output_string: false,// if you want the output to be in string form or if you want it to just print to the console

    };
    
    
    let mut f = screen{
        chars: vec!("0".to_string(),"1".to_string()), // these are the different ascii "items" that get renderd X and Y are the cordinates 
        x: vec!(1,2),
        y: vec!(1,2),
        delay: 10,// delay between each frame for
        sprite: Vec::new(),// the sprite vector contains all the sprites that are going to be renderd 

    };
    
    let mut a = x.setup(); // set up the Acore struct 
    //! map loading stuff
    f.loadmap(load("./maps/map.rmap")); // loads in the map
    let player = f.findX("@".to_string()); // gets the player position in the screen.chars section findAllOfX works the same but returns a vector 
    f.loadmap(toMap("#####\n33333".to_string()));//if you want to make a map out of a string 

    //! physics stuff
    a.addphysics(player); // adds phycis to an object 
    a.addphysicsForAllX(&f,"I".to_string());// adds physics to all objects with the correct char
    a.removephysics(player); // remove physics form an object 
    a.changePhysics(-1); // change the gravity constant to make gravity stronger or weaker or upside down 
    let mut sprite = sprite::load("./maps/sprite.rsprite"); // loads the sprite from a text file 
    f.sprite.push(sprite);//push the sprite to the sprite vector 
    f.sprite[0].setxy(2,-2);//sets the position of the sprite 
    loop {
        
        cursor::clear();// clear the screen
        a.render(&mut f);// renders it all does not work with Bcore
        println!("{}",a.render(&mut f););// if you are using Bcore the output gets output in a string ( Does not work with Acore )
        f.sprite[0].movexy(1, 0); // moves the sprite to the side 

        if keyin::keydown(){ // checks if there is a keydown

        
            match keyin::getkey(){// gets said keycode
                97 =>{
                    f.x[player as usize] -= 1;
                }
                100 =>{
                    f.x[player as usize] += 1;
                }
                119 =>{
                    f.y[player as usize] -= 1;
                }
                115 =>{
                    f.y[player as usize] += 1;
                }
                _ =>{

            }
        }
      

    }

  
}
``` 