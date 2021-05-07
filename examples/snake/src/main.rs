extern crate ccdb;
use ccdb::core::{Core, Screen,backend}; // there are 2 diffrent Cores there is Core and there is banana both work the same way when talking to them
use ccdb::collision; // for collision
use ccdb::cursor; // for moving the cursor
use ccdb::keycode; // For key input
use ccdb::keyin; // For key input
use ccdb::loader;
use ccdb::loader::to_map; // this is the loader which makes it so you can load a map from file or load a map from string
use ccdb::terminal;
use ccdb::ui; // ui library
use rand::prelude::*;

pub fn main() {
    cursor::clear(); // clears the Screen
    cursor::hide_cursor(); // hides the cursor

    // not really needed here but its so you understand how to use it
    let assets = loader::load_from_folder(
        "./ast/".to_string(),
        ".rmap".to_string(),
        ".rsprite".to_string(),
    ); // get a folder of maps and sprites
    let map1 = assets.load_asset_map("map").unwrap(); //load an assets from the folder struct

    let x = Core {
        // set up the Core
        name: "Snake".to_string(),                 //name
        desc: " - Snake example game".to_string(), // descirption
        line_lenght: 40,                           // ammount of chars per line
        lines: 10,                                 //amount of lines
        debug: false,                              //debug
        threads: 2,                                // ammount of threads
        output_string: false, // if you want the output in string form or it just to printed out to the console directly
        backend: backend::a,
    };
    // if the map is not loaded properly it will just say error
    let mut f = Screen {
        // Screen is where everything will be stored and how it should be renderd
        chars: vec!["Error".to_string()],
        x: vec![1],
        y: vec![1],
        delay: 10,          // delay between each frame
        sprite: Vec::new(), // this is where all sprites go
        orgin: map1.clone(),
    };

    let mut a = x.setup(); // setup the Core

    // set terminal size so that the window is not huge
    terminal::set_terminal_size(60, 20); // change terminal size

    // create 2 buttons and add them to the ui component
    // get the ui started
    let mut ui = ui::UI::new();

    let mut button = ui::Button::new(0, "Play again");
    let mut button2 = ui::Button::new(1, "Quit");

    ui.buttons.push(button);
    ui.buttons.push(button2);

    // some variables
    let mut rendui = false;
    let mut change_x = 0;
    let mut change_y = 0;
    let mut cur = 0;
    let speed = 10; //lower is faster
    let mut snake: Vec<[i64; 2]> = vec![[10, 3]];

    let mut x = 10;
    let mut y = 3;
    let mut snek_lenght = 1;
    let mut foodpos: [i64; 2] = [3, 4];
    f.load_map(map1.clone());
   
    loop {
        f.return_to_orgin();
        cur += 1; // keep the speed under check
        if cur >= speed + 1 {
            cur = 0;
        }

        cursor::clear(); // revert back the cursor
        cursor::hide_cursor();
        for x in snake.iter() {
            // push all snake parts
            f.x.push(x[0]);
            f.y.push(x[1]);
            f.chars.push("@".to_string());
        }
        // print out the apple
        f.x.push(foodpos[0]);
        f.y.push(foodpos[1]);
        f.chars.push("A".to_string());

        if !rendui {
            println!("{}", a.render(&mut f)); //render the entire Screen
            println!("Score: {}          ", snek_lenght - 1);
        }
        if rendui {
            f.load_map(to_map(" \n".to_string())); // clear the vector
            a.render(&mut f);
            println!("GAME OVER");
            ui.rend(); // render the ui
            match ui.buttons[0].get_status() {
                // getting if a button is pressed down or hoverd over or not
                ui::ButtonAction::Press => {
                    x = 10;
                    y = 3;
                    snake = vec![[10, 3]];
                    snek_lenght = 1;
                    foodpos = [3, 4];
                    change_x = 0;
                    change_y = 0;
                    rendui = false;
                    cursor::clear(); // revert back the cursor
                    a.render_blank(&mut f);
                    a.render_blank(&mut f);

                }
                _ => {}
            }
            match ui.buttons[1].get_status() {
                // getting if a button is pressed down or hoverd over or not
                ui::ButtonAction::Press => {
                    break;
                }
                _ => {}
            }
        }

        if cur == speed {
            // makes sure its not too fast
            x += change_x;
            y += change_y;
            snake.push([x, y]);
        }
        // keep it in shape
        if snake.len() > snek_lenght {
            snake.remove(0);
        }
        // check if the apple has collided with something aka the snake
        let mut apple_col = false;
        if collision::get_collision(f.find_X("A".to_string()) as usize, &f.gmap()) {
            snek_lenght += 1; // make snake longer
            foodpos = new_apple(); // get new apple positions
            apple_col = true;
        }
        // check if the snake has collided with something then makes sure that thing is not the apple
        let all_x = f.find_all_of_X("@".to_string());
        let mut len = 0;
        if all_x.len() >= 1 {
            if all_x.len() > 1 {
                len = all_x.len() - 1;
            }
            if collision::get_collision(all_x[len] as usize, &f.gmap()) && !apple_col {
                rendui = true;
            }
        }

        // keyinput stuff
        
            match keyin::get_key() {
                keycode::A => {
                    change_x = -1;
                    change_y = 0;
                }
                keycode::D => {
                    change_x = 1;
                    change_y = 0;
                }
                keycode::W => {
                    change_x = 0;
                    change_y = -1;
                }
                keycode::S => {
                    change_y = 1;
                    change_x = 0;
                }

                _ => {}
            }
        
    }
}
fn new_apple() -> [i64; 2] {
    let mut rng = thread_rng();
    let x = rng.gen_range(1.0..29.0) as i64;
    let y = rng.gen_range(1.0..9.0) as i64;
    [x, y]
}
