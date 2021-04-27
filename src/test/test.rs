
use crate::acore::{Core, Screen}; // there are 2 diffrent cores there is Core and there is banana both work the same way when talking to them
use crate::collision; // for collision
use crate::cursor; // for moving the cursor
use crate::keycode; // For key input
use crate::keyin; // For key input
use crate::loader;
use crate::loader::{load, map, to_map}; // this is the loader which makes it so you can load a map from file or load a map from string
use crate::sprite; // for sprites
use crate::terminal;
use crate::ui;
use std::time::{Duration, Instant};
use std::{thread, time};




pub fn t() {
    cursor::clear(); // clears the screen
    cursor::hide_cursor(); // hides the cursor
    
    
    //let mut psprite = assets.loadAssetSprite("sprite").unwrap();

    let x = Core {
        name: "Test project".to_string(),
        desc: " - A test project".to_string(),
        line_lenght: 40,
        lines: 10,
        debug: true,
        threads: 10,
        output_string: false,
    };

    let mut f = Screen {
        chars: vec!["0".to_string(), "1".to_string()],
        x: vec![1, 2],
        y: vec![1, 2],
        delay: 0,
        sprite: Vec::new(),
        orgin: map::new(),
    };

    let mut a = x.setup(); //load("./maps/map.rmap")load("./maps/map.rmap")
    f.load_map(to_map(" \n".to_string()));
    let run = true;
    let player = f.find_X("@".to_string());
    //f.loadmap(toMap("#####\n33333".to_string()));
    a.add_physics(player);
    a.add_physics_for_all_X(&f, "I".to_string());
    a.remove_physics(player);
    // a.removephysForAllX(&f,"I".to_string());
    a.change_physics(1);
    let mut sprite = to_map("#\n".to_string()).to_sprite();
    f.sprite.push(sprite);
    f.sprite[0].set_xy(2,-2);
    f.sprite[0].move_xy(5, 0);

    let mut dir = 0;
    terminal::set_terminal_size(50, 20);
    let Button = ui::Button::new(0, "test");
    let Button2 = ui::Button::new(1, "test2");

    let mut ui = ui::UI::new();
    ui.buttons.push(Button);
    ui.buttons.push(Button2);
   // terminal::set_terminal_name("test");
    
        f.return_to_orgin();
        cursor::clear();
        //a.render_blank(&mut f);
       // cursor::clear();
        cursor::hide_cursor();

        //cursor::gotoxy(100,100);
        a.render(&mut f);
        //cursor::clear();
        for x in f.chars.iter() {
            //print!("t{}",x);
        }

        ui.rend();

        match ui.buttons[0].get_status(){
            ui::ButtonAction::Press =>{
                println!("yes     ");
            }
            ui::ButtonAction::Hover =>{
                println!("maybe    ");
            }
            ui::ButtonAction::Idle =>{
                println!("no not  ");
            }
            _ =>{

            }
        }



        if keyin::key_down() {
            match keyin::get_key() {
                keycode::A => {
                    f.x[player as usize] -= 1;
                    if collision::get_collision(player as usize, &f.gmap()) {
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
pub fn check_const(){
    use crate::acore;
    use crate::bcore;
    let x = acore::Core {
        name: "Test project".to_string(),
        desc: " - A test project".to_string(),
        line_lenght: 40,
        lines: 10,
        debug: true,
        threads: 10,
        output_string: false,
    };

    let mut f = acore::Screen {
        chars: vec!["0".to_string(), "1".to_string()],
        x: vec![1, 2],
        y: vec![1, 2],
        delay: 0,
        sprite: Vec::new(),
        orgin: map::new(),
    };
    let mut buildA = x.setup();

    let X = bcore::Core {
        name: "Test project".to_string(),
        desc: " - A test project".to_string(),
        line_lenght: 40,
        lines: 10,
        debug: true,
        threads: 10,
        output_string: false,
    };

    let mut S = bcore::Screen {
        chars: vec!["0".to_string(), "1".to_string()],
        x: vec![1, 2],
        y: vec![1, 2],
        delay: 0,
        sprite: Vec::new(),
        orgin: map::new(),
    };
    let mut buildB = X.setup();
    
    assert_eq!(buildA.render(&mut f),buildB.render(&mut S));
}