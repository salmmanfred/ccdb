# ccdb:  The cmd game engine  
(Thats also multi threaded)  
  
# How to use  
  
SOME FEATURES ARE ONLY TESTED ON WINDOWS THIS INCLUDES KEYIN AND CURSOR!(these are made in C)  
  
  
## Learn how to use it by example  
  
  
```rust  
// core v0.1.0
extern crate ccdb;
use ccdb::acore::{Core,Screen}; // there are 2 diffrent Cores there is Core and there is banana both work the same way when talking to them 
use ccdb::loader::{load,to_map};// this is the loader which makes it so you can load a map from file or load a map from string 
use ccdb::loader;
use ccdb::keyin; // For key input
use ccdb::keycode; // For key input
use ccdb::cursor; // for moving the cursor
use ccdb::sprite; // for sprites
use ccdb::collision; // for collision
use ccdb::ui;// ui library 
use ccdb::terminal;



pub fn main() {
    
    cursor::clear(); // clears the Screen
    cursor::hide_cursor(); // hides the cursor
    let assets = loader::load_from_folder("./maps/".to_string(),".rmap".to_string(),".rsprite".to_string()); // get a folder of maps and sprites 
    let map1 = assets.load_asset_map("map").unwrap();//load an assets from the folder struct 
    let psprite = assets.load_asset_sprite("sprite").unwrap();//


    let x = Core{// set up the Core
        name: "Test project".to_string(),//name 
        desc: " - A test project".to_string(),// descirption
        line_lenght: 40,// ammount of chars per line
        lines: 8,//amount of lines
        debug: true,//debug 
        threads: 4,// ammount of threads
        output_string: false,// if you want the output in string form or it just to printed out to the console directly 
    };
    
    
    let mut f = Screen{// Screen is where everything will be stored and how it should be renderd 
        chars: vec!("0".to_string(),"1".to_string()),
        x: vec!(1,2),
        y: vec!(1,2),
        delay: 10,// delay between each frame
        sprite: Vec::new(),// this is where all sprites go
    };
    
    let mut a = x.setup();// setup the Core
    f.load_map(map1);//load a map from a map struct 
    f.load_map(to_map("#####\n33333".to_string()));//if you want to make a map out of a string 
    f.load_map(load("./maps/map.rmap"));
    let run = true;
    let player = f.find_X("@".to_string());// find a chars position
    
    a.add_physics(player);// add physics to char position
    a.add_physics_for_all_X(&f,"I".to_string());
    a.remove_physics(player);//remove physics for char pos
    a.change_physics(1);// change the phycis 
    
    let mut sprite = psprite;// sprite loaded from assets otherwise its sprite::load("./maps/sprite.rsprite");
    f.sprite.push(sprite);//add the sprite to the sprite rendering list 
    f.sprite[0].set_xy(2,-2);// set the sprites position
    f.sprite[0].move_xy(5, 0);// move the sprite

    let x = terminal::get_terminal_size();// get the terminal size
    terminal::set_terminal_size(50,20);// change terminal size


    // create 2 buttons and add them to the ui component 
    let button = ui::button::new(0,"test");
    let button2 = ui::button::new(1,"test2");

    let mut ui = ui::UI::new();
    ui.buttons.push(button);
    ui.buttons.push(button2);
    

    let mut dir = 0;
    loop {
        cursor::clear();
        cursor::hide_cursor();
   
        
        println!("{}",a.render(&mut f));//render the entire Screen

        
        
        ui.rend();// render the ui
        match ui.buttons[0].get_status(){// getting if a button is pressed down or hoverd over or not 
            ui::button_action::Press =>{
                println!("yes");
            }
            ui::button_action::Hover =>{
                println!("maybe");
            }
            ui::button_action::Idle =>{
                println!("no not");
            }
            _ =>{

            }
        }
        
        
        //a.render(&mut f);

       
        

        f.sprite[0].move_xy(1, 0);
        
        //standard key input system
        if keyin::key_down(){

        
            match keyin::get_key(){
                keycode::A =>{
                    f.x[player as usize] -= 1;
                    if collision::get_collision(player as usize, &f.gmap())//how to get the collision must pass in the Screen
                    {
                        f.x[player as usize] += 1;
                    }
                }
                keycode::D =>{
                    f.x[player as usize] += 1;
                    if collision::get_collision(player as usize, &f.gmap())
                    {
                        f.x[player as usize] -= 1;
                    }
                }
                keycode::W =>{
                    f.y[player as usize] -= 1;
                    if collision::get_collision(player as usize, &f.gmap())
                    {
                        f.y[player as usize] += 1;
                    }
                }
                keycode::S =>{
                    f.y[player as usize] += 1;
                    if collision::get_collision(player as usize, &f.gmap())
                    {
                        f.y[player as usize] -= 1;
                    }
                }
                keycode::SPACE =>{
                    for x in 0..10{
                        f.x[player as usize] += 1;
                        if collision::get_collision(player as usize, &f.gmap())
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
  
## Differance between BCore and ACore  
Core is mutli threaded and does not work very efficiently with big amount of text  
BCore is the old algorithm for making the text  
Their names are ACore: Banana Core, BCore: Olive Core  
BCore is faster than ACore but BCore is just weird  

## UI  
There is an issue where the keyboard things get really slow when using the ui    
Try to not use them at the same time  
  
# Multi threading  
If you want to use multi threading you have to use ACore  
The amount of threads cannot be odd  
If you have more threads than lines the program will crash  
  
# Contributing  
If you have linux it would be nice if you could test the keyin and cursor libraries to see if they will work on linux!  
  
# Roadmap  
v0.2.0: Add a function to find all of a certain character or just the first one  DONE  
v0.3.0: Being able to get the output in a string instead of the cmd   DONE  
v0.4.0: Physics and collision  DONE  
v0.5.0: Loading of ascii sprites from file  DONE  
v0.6.0: Key input rework  DONE (+ some rework to the aCore(IT works allot better))  
v0.7.0: Rework of variable names and function names  DONE  
v0.8.0: Adding a way to load in a folder  DONE  
v0.9.0: Getting the code ready for 1.0.0  DONE  
v1.0.0: optimization  DONE  
v1.1.0: Terminal control  DONE  
v1.2.0: UI   DONE
v1.3.0: No plans  
v1.4.0: No plans  
