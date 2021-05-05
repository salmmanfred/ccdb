# How to use  
  
SOME FEATURES ARE ONLY TESTED ON WINDOWS THIS INCLUDES KEYIN AND CURSOR!(these are made in C)  
  
  
## Learn how to use it by example  
  
  
```rust  
extern crate ccdb;
use ccdb::core::{Core, Screen,backend}; // there are 2 diffrent Cores there is Core and there is banana both work the same way when talking to them
use ccdb::collision; // for collision
use ccdb::cursor; // for moving the cursor
use ccdb::keycode; // For key input
use ccdb::keyin; // For key input
use ccdb::loader;
use ccdb::loader::{load, map, to_map}; // this is the loader which makes it so you can load a map from file or load a map from string
use ccdb::sprite; // for sprites
use ccdb::terminal;
use ccdb::ui; // ui library
use ccdb::particle;

pub fn main() {
    cursor::clear(); // clears the Screen
    cursor::hide_cursor(); // hides the cursor
    let assets = loader::load_from_folder(
        "./maps/".to_string(),
        ".rmap".to_string(),
        ".rsprite".to_string(),
    ); // get a folder of maps and sprites
    let map1 = assets.load_asset_map("map").unwrap(); //load an assets from the folder struct
    let psprite = assets.load_asset_sprite("sprite").unwrap(); //

    let x = Core {
        // set up the Core
        name: "Test project".to_string(),      //name
        desc: " - A test project".to_string(), // descirption
        line_lenght: 40,                       // ammount of chars per line
        lines: 8,                              //amount of lines
        debug: true,                           //debug
        threads: 4,                            // ammount of threads
        output_string: false, // if you want the output in string form or it just to printed out to the console directly
        backend: backend::A, // there are also backend::B and backend::N (which is null)
    };

    let mut f = Screen {
        // Screen is where everything will be stored and how it should be renderd
        chars: vec!["0".to_string(), "1".to_string()],
        x: vec![1, 2],
        y: vec![1, 2],
        delay: 10,          // delay between each frame
        sprite: Vec::new(), // this is where all sprites go
        orgin: map::new(),
    };

    let mut water = particle::water{ // create the water struct 
        droplets: Vec::new(),//leave this empty
        chars: "W".to_string(),// the character the water is based of off.
    };
    water.collect_drop(f.gmap()); // after you have loaded the map you can get started adding the droplets.
    

   let mut a = x.setup(); // setup the Core
    f.load_map(map1.clone()); //load a map from a map struct
    f.load_map(to_map("#####\n33333".to_string())); //if you want to make a map out of a string
    f.load_map(load("./maps/map.rmap"));
    let run = true;
    let player = f.find_X("@".to_string()); // find a chars position

    a.add_physics(player); // add physics to char position
    a.add_physics_for_all_X(&f, "I".to_string());
    a.remove_physics(player); //remove physics for char pos
    a.change_physics(1); // change the phycis

    let mut sprite = psprite; // sprite loaded from assets otherwise its sprite::load("./maps/sprite.rsprite");
    f.sprite.push(sprite); //add the sprite to the sprite rendering list
    f.sprite[0].set_xy(2, -2); // set the sprites position
    f.sprite[0].move_xy(5, 0); // move the sprite

   // let x = terminal::get_terminal_size(); // get the terminal size
    terminal::set_terminal_size(50, 20); // change terminal size

    // create 2 buttons and add them to the ui component
    let button = ui::Button::new(0, "test");
    let button2 = ui::Button::new(1, "test2");

    let mut ui = ui::UI::new();
    ui.buttons.push(button);
    ui.buttons.push(button2);
    f.set_orgin(map1.clone()); // set the orgin

    loop {
        f.return_to_orgin(); // use this if you want to return to the orgin
        a.render_blank(&mut f); //render a output the same size as the normal rend just with nothing in it

        cursor::clear();
        cursor::hide_cursor();

        println!("{}", a.render(&mut f)); //render the entire Screen
        f.load_map(water.run(f.gmap())); // to render the water just must give it the current map from the screen by doing screen.gmap(); 
        // then you can load the map that it returns straight into the screen or store it for later use. 
        ui.rend(); // render the ui



        match ui.buttons[0].get_status() {
            // getting if a button is pressed down or hoverd over or not
            ui::ButtonAction::Press => {
                println!("yes");
            }
            ui::ButtonAction::Hover => {
                println!("maybe");
            }
            ui::ButtonAction::Idle => {
                println!("no not");
            }
            _ => {}
        }

        //a.render(&mut f);

        f.sprite[0].move_xy(1, 0);

        //standard key input system
        
            match keyin::get_key() {
                keycode::A => {
                    f.x[player as usize] -= 1;
                    if collision::get_collision(player as usize, &f.gmap())
                    //how to get the collision must pass in the Screen
                    {
                        f.x[player as usize] += 1;
                    }
                }
                keycode::D => {
                    f.x[player as usize] += 1;
                    if collision::get_collision(player as usize, &f.gmap()) {
                        f.x[player as usize] -= 1;
                    }
                }
                keycode::W => {
                    f.y[player as usize] -= 1;
                    if collision::get_collision(player as usize, &f.gmap()) {
                        f.y[player as usize] += 1;
                    }
                }
                keycode::S => {
                    f.y[player as usize] += 1;
                    if collision::get_collision(player as usize, &f.gmap()) {
                        f.y[player as usize] -= 1;
                    }
                }
                keycode::SPACE => {
                    for x in 0..10 {
                        f.x[player as usize] += 1;
                        if collision::get_collision(player as usize, &f.gmap()) {
                            f.x[player as usize] -= 1;
                            break;
                        }
                    }
                }
                _ => {}
            }
        
    }
}



```  
  
## Differance between A and B backend  
A is  Multi threaded while BCore is single threaded  
B is the old algorithm for making the text  
Their names are A: Banana Backend, B: Olive Backend  

## UI  
There is an issue where the keyboard things get really slow when using the ui    
Try to not use them at the same time  
  