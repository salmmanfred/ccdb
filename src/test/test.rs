use crate::core::{Core, Screen,backend}; // there are 2 diffrent cores there is Core and there is banana both work the same way when talking to them
use crate::collision; // for collision
use crate::cursor; // for moving the cursor
use crate::keycode; // For key input
use crate::keyin; // For key input
use crate::loader;
use crate::loader::{load, map, to_map}; // this is the loader which makes it so you can load a map from file or load a map from string
use crate::particle;
use crate::sprite; // for sprites
use crate::terminal;
use crate::ui;
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
        backend: backend::a,
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

    let player = f.find_X("@".to_string());
    //f.loadmap(toMap("#####\n33333".to_string()));
    a.add_physics(player);
    a.add_physics_for_all_X(&f, "I".to_string());
    a.remove_physics(player);
    // a.removephysForAllX(&f,"I".to_string());
    a.change_physics(1);
    let sprite = to_map("#\n".to_string()).to_sprite();
    f.sprite.push(sprite);
    f.sprite[0].set_xy(2, -2);
    f.sprite[0].move_xy(5, 0);

    terminal::set_terminal_size(50, 20);
    let button = ui::Button::new(0, "test");
    let button2 = ui::Button::new(1, "test2");

    let mut ui = ui::UI::new();
    ui.buttons.push(button);
    ui.buttons.push(button2);
    // terminal::set_terminal_name("test");

    f.return_to_orgin();
    cursor::clear();
    //a.render_blank(&mut f);
    // cursor::clear();
    cursor::hide_cursor();

    //cursor::gotoxy(100,100);
    a.render(&mut f);
    //cursor::clear();
    let mut water = particle::water {
        // create the water struct
        droplets: Vec::new(),   //leave this empty
        chars: "W".to_string(), // the character the water is based of off.
    };
    water.collect_drop(f.gmap()); // after you have loaded the map you can get started adding the droplets.
    water.run(f.gmap());
    ui.rend();

    match ui.buttons[0].get_status() {
        ui::ButtonAction::Press => {
            println!("yes     ");
        }
        ui::ButtonAction::Hover => {
            println!("maybe    ");
        }
        ui::ButtonAction::Idle => {
            println!("no not  ");
        }
    }

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
pub fn check_const() {
    use crate::core;
 ;
    let x = core::Core {
        name: "Test project".to_string(),
        desc: " - A test project".to_string(),
        line_lenght: 40,
        lines: 10,
        debug: true,
        threads: 10,
        output_string: false,
        backend: backend::b,

    };

    let mut f = core::Screen {
        chars: vec!["0".to_string(), "1".to_string()],
        x: vec![1, 2],
        y: vec![1, 2],
        delay: 0,
        sprite: Vec::new(),
        orgin: map::new(),
    };
    let mut build_a = x.setup();

    let x2 = core::Core {
        name: "Test project".to_string(),
        desc: " - A test project".to_string(),
        line_lenght: 40,
        lines: 10,
        debug: true,
        threads: 10,
        output_string: false,
        backend: backend::a,
    };

    let mut s = core::Screen {
        chars: vec!["0".to_string(), "1".to_string()],
        x: vec![1, 2],
        y: vec![1, 2],
        delay: 0,
        sprite: Vec::new(),
        orgin: map::new(),
    };
    let mut build_b = x2.setup();

    assert_eq!(build_a.render(&mut f), build_b.render(&mut s));
}
