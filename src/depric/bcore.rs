//! Olive backend
// Honestly would remove it if it wasent so damn good at its job some how

use crate::loader;
use crate::physics;
use crate::sprite;
use std::time::Duration;
use std::{thread, time};

// Core Cort and Screen is the same as in banana(mostly)

pub struct Core {
    pub name: String,
    pub desc: String,
    pub line_lenght: i64,
    pub lines: i64,
    pub debug: bool,
    pub threads: i8,
    pub output_string: bool,
}
pub struct Cort {
    name_desc: String,
    v: i64,
    pub char_x_line: i64,
    pub lines: i64,
    prevmap: loader::map,
    renderd: String,
    physobj: Vec<i64>,
    debug: bool,
    gravity: i64,
    output_string: bool,
}
/*
PREVX: Vec<i64>,
    PREVY: Vec<i64>,
    PREVG: Vec<String>,
    */

pub struct Screen {
    pub chars: Vec<String>,
    pub x: Vec<i64>,
    pub y: Vec<i64>,
    pub delay: u64,
    pub sprite: Vec<sprite::sprite>,
    pub orgin: loader::map,
}
impl Core {
    pub fn setup(&self) -> Cort {
        self.setup001()
    }
    fn setup001(&self) -> Cort {
        let xx = &self.name.to_string();
        let mut xxs = xx.to_string();
        xxs.push_str(&self.desc);
        if self.debug {
            xxs.push_str(" CCDB BCore");
        }
        //println!("{}",xxs);
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
            renderd: "".to_string(),
            physobj: Vec::new(),
            debug: self.debug,
            gravity: 1,
            output_string: self.output_string,
        }
    }
}

impl Cort {
    fn equall(&self, map: loader::map) -> bool {
        if self.prevmap.x == map.x && self.prevmap.y == map.y && self.prevmap.chars == map.chars {
            return true;
        }

        return false;
    }
    pub fn render(&mut self, Screen: &mut Screen) -> String {
        // println!("■".green())

        thread::sleep(time::Duration::from_millis(Screen.delay));
        physics::b_render_physics(Screen, self.physobj.clone(), self.gravity);
        println!("{}", self.name_desc);

        if !self.equall(Screen.gmap()) {
            // sends it to the Screen for render

            self.prevmap = Screen.run(self.lines, self.char_x_line, self);
        }
        if self.output_string == false {
            println!("{}", self.renderd);
        } else {
            return self.renderd.clone();
        }
        "".to_string()
    }
    pub fn render_blank(&mut self, screen: &mut Screen) {
        let map = screen.gmap();
        screen.load_map(loader::to_map(" \n".to_string()));
        screen.run(self.lines, self.char_x_line, self);
        screen.x = map.x;
        screen.y = map.y;
        screen.chars = map.chars;
    }
    fn prvrend(&mut self, f: String) {
        // for putting the previous render in the self.renderd
        self.renderd = f;
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
        // removes physics
        self.physobj.retain(|&x| x != pos);
    }
    pub fn remove_physics_for_all_X(&mut self, Screen: &Screen, chr: String) {
        // removes allot of objects to be renderd for phycis
        for x in Screen.find_all_of_X(chr) {
            self.remove_physics(x);
        }
    }
    pub fn change_physics(&mut self, grav: i64) {
        // change the physics to be what ever you want it to be
        self.gravity = grav;
    }
}
impl Screen {
    pub fn run(&self, size: i64, size2: i64, Cort: &mut Cort) -> loader::map {
        let mut line: String = "".to_string();
        let mut X = self.x.clone(); // load in a sprite
        let mut Y = self.y.clone();
        let mut C = self.chars.clone();

        for x in 0..self.sprite.len() {
            // does the actually loading
            C.extend(self.sprite[x].chars.clone());
            X.extend(self.sprite[x].x.clone());
            Y.extend(self.sprite[x].y.clone());
        }

        for sx in 0..size {
            // splits everything into a bunch of X lines then calls make line to make said x lines
            let mut betterx: Vec<i64> = Vec::new();
            let mut bettern: Vec<String> = Vec::new();
            //betterx.remove()
            // splits it into the correct y section
            for x in 0..C.len() {
                if sx == Y[x] {
                    betterx.push(X[x]);
                    bettern.push(C[x].clone());
                }
            }
            /* for x in 0..self.sprite.len(){
                if self.sprite[x].y[x] == sx{
                }
            }*/
            // makes the line and prints it
            line.push_str(&format!("{}\n", &self.make_line(betterx, bettern, size2)));
        }
        Cort.prvrend(line);
        loader::map {
            x: X.clone(),
            y: Y.clone(),
            chars: C.clone(),
        }
    }
    pub fn make_line(&self, betterx: Vec<i64>, bettern: Vec<String>, size: i64) -> String {
        let mut vc: Vec<&str> = Vec::new();
        // really complicated version of what was in banana
        for i in 0..size {
            // size is the lenght of the entire line
            let masi = i;
            let mut rus = 1;
            let mut run = true;
            for i in 0..betterx.len() {
                // finds the correct char?

                if betterx[i] == masi {
                    vc.push(&bettern[i]);
                    rus = 0;
                    break;
                }
            }
            if run {
                if rus == 1 {
                    vc.push(" ");
                }
                if i == 0 {
                    vc.push("");
                } /*else if i == size-1{

                      vc.push("");
                      break;

                  }*/
            }
        }
        vc.into_iter().collect::<String>()
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
    pub fn find_X(&self, ch: String) -> i64 {
        for x in 0..self.chars.len() {
            if self.chars[x] == ch {
                return x as i64;
            }
        }
        0
    }
    pub fn find_all_of_X(&self, ch: String) -> Vec<i64> {
        let mut all: Vec<i64> = Vec::new();
        for x in 0..self.chars.len() {
            if self.chars[x] == ch {
                all.push(x as i64);
            }
        }
        all
    }
    pub fn push_char(&mut self, x: i64, y: i64, char: &str) {
        self.x.push(x);
        self.y.push(y);
        self.chars.push(char.to_string());
    }
}
