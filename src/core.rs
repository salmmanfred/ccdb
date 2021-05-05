use crate::a;
use crate::b;

// ! core
use crate::loader;
use std::time::Duration;
use std::{thread, time};
//use crate::colour;
use crate::check::thread_check;
use crate::physics;
use crate::sprite;
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
use std::time::Instant;

//use std::sync::{Mutex, Arc};
//use crate::messages::{message,listen};
#[derive(Clone)]
pub enum backend{
    a,
    b,
    n,
}
//this is the core used for things like declaring the line lenght and amount of lines
pub struct Core {
    pub name: String,
    pub desc: String,
    pub line_lenght: i64,
    pub lines: i64,
    pub debug: bool,
    pub threads: i8,
    pub output_string: bool,
    pub backend: backend,
}
// this is what core "compiles" into so that the core can use the data easier
pub struct Cort {
    name_desc: String,
    v: i64,
    pub char_x_line: i64,
    pub lines: i64,
    // prevmap is used so you dont render the same thing more than once saving some cpu usage
    prevmap: loader::map,
    pub thr: i8,
    physobj: Vec<i64>,
    gravity: i64,
    debug: bool,
    pub renderd: String,
    pub output_string: bool,
    pub extime: i64,
    backend: backend,
}

// Screen is the Screen which stores the current map data
pub struct Screen {
    pub chars: Vec<String>,
    pub x: Vec<i64>,
    pub y: Vec<i64>,
    pub delay: u64,
    pub sprite: Vec<sprite::sprite>,
    pub orgin: loader::map,
}

// used for the set up of Cort
impl Core {
    pub fn setup(&self) -> Cort {
        self.setup001()
    }
    fn setup001(&self) -> Cort {
        // compiles core into Cort where data and meta data is stored
        let xx = &self.name.to_string();
        let mut xxs = xx.to_string();
        xxs.push_str(&self.desc);
        if self.debug {
            xxs.push_str(" CCDB CORE");
        }
       
      
        thread_check(self.threads, self.lines);
        Cort {
            name_desc: xxs,
            v: 1,
            char_x_line: self.line_lenght,
            lines: self.lines,
            prevmap: loader::map {
                x: Vec::new(),
                y: Vec::new(),
                chars: Vec::new(),
            },
            thr: self.threads,
            physobj: Vec::new(),
            debug: self.debug,
            gravity: 1,
            renderd: "".to_string(),
            output_string: self.output_string,
            extime: 0,
            backend: self.backend.clone(),
        }
    }
}

impl Cort {
    fn equall(&self, map: loader::map) -> bool {
        if self.prevmap.x == map.x && self.prevmap.y == map.y && self.prevmap.chars == map.chars {
            return true; // checks if the Screen and a map is the same
        }

        return false;
    }
    pub fn render(&mut self, Screen: &mut Screen) -> String {
        // ■ honestly what the fuck is this

        thread::sleep(time::Duration::from_millis(Screen.delay));

        println!("{}", self.name_desc);

        physics::render_physics(Screen, self.physobj.clone(), self.gravity);
        if !self.equall(Screen.gmap()) {
            // helps preformance by not rendering the same window again and again and again
            self.prevmap = Screen.run(self); // starts the Screen rendering

        } else {

            if !self.output_string {
                println!("{}", self.renderd)
            }

        }
        // just because i found it better this way.

        if self.output_string {
            return self.renderd.clone();
        } else {
            return "".to_string();
        }
    }
    pub fn render_blank(&mut self, screen: &mut Screen) {
        let map = screen.gmap();
        screen.load_map(loader::to_map(" \n".to_string()));
        screen.run(self);
        screen.x = map.x;
        screen.y = map.y;
        screen.chars = map.chars;
    }

    pub fn add_physics(&mut self, pos: i64) {
        // adds a object to be renderd for phycis later
        self.physobj.push(pos);
    }

    pub fn add_physics_for_all_X(&mut self, Screen: &Screen, chr: String) {
        // adds allot of objects to be renderd for phycis later
        for x in Screen.find_all_of_X(chr) {
            self.physobj.push(x);
        }
    }

    pub fn remove_physics(&mut self, pos: i64) {
        // removes physics from object
        self.physobj.retain(|&x| x != pos);
    }

    pub fn remove_physics_for_all_X(&mut self, Screen: &Screen, chr: String) {
        // removes allot of objects to be renderd for phycis
        for x in Screen.find_all_of_X(chr) {
            self.remove_physics(x);
        }
    }

    pub fn change_physics(&mut self, grav: i64) {
        self.gravity = grav;
    }
}
impl Screen {
    pub fn run(&self, cort: &mut Cort) -> loader::map {
        match cort.backend {
            backend::a => return a::run(self, cort),
            backend::b => return b::run(self, cort.lines, cort.char_x_line, cort),
            backend::n => {
                println!("no backend selected");
                return self.gmap();
            }
            _ => {
                panic!("NO BACKEND FOUND")
            }
        }

       // self.gmap()
    }

    pub fn load_map(&mut self, map: loader::map) {
        // for loading a map into the Screen
        self.x = map.x.clone();
        self.y = map.y.clone();
        self.chars = map.chars.clone();
        //self.orgin = map.clone(); // set the orgin
    }
    pub fn set_orgin(&mut self, map: loader::map) {
        self.orgin = map;
    }
    pub fn return_to_orgin(&mut self) {
        self.load_map(self.orgin.clone());
    }
    pub fn cgmap(&self) -> loader::map {
        // clean get map
        let mut charss = self.chars.clone();
        let mut xxx = self.x.clone();
        let mut yyy = self.y.clone();
        loader::map {
            x: xxx,
            y: yyy,
            chars: charss,
        }
    }
    pub fn gmap(&self) -> loader::map {
        // makes a map out of the current data in Screen

        let mut charss = self.chars.clone();
        let mut xxx = self.x.clone();
        let mut yyy = self.y.clone();

        for x in 0..self.sprite.len() {
            //appends the sprite
            charss.extend(self.sprite[x].chars.clone());
            xxx.extend(self.sprite[x].x.clone());
            yyy.extend(self.sprite[x].y.clone());
        }
        loader::map {
            x: xxx,
            y: yyy,
            chars: charss,
        }
    }
    pub fn push_char(&mut self, x: i64, y: i64, char: &str) {
        self.x.push(x);
        self.y.push(y);
        self.chars.push(char.to_string());
    }
    pub fn find_X(&self, ch: String) -> i64 {
        // for finding a character
        for x in 0..self.chars.len() {
            if self.chars[x] == ch {
                return x as i64;
            }
        }
        0
    }
    pub fn find_all_of_X(&self, ch: String) -> Vec<i64> {
        // returns a vector of the position of all instanses of a certain character
        let mut all: Vec<i64> = Vec::new();
        for x in 0..self.chars.len() {
            if self.chars[x] == ch {
                all.push(x as i64);
            }
        }
        all
    }
}
