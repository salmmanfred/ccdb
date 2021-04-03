
use std::time::Duration;
use std::{thread, time};
use crate::loader;
use crate::colour;
use crate::cursor;
//this is the core used for things like declaring the line lenght and amount of lines
pub struct core{
    pub name: String,
    pub desc: String,
    pub linelenght: i64,
    pub lines: i64
}
// this is what core "compiles" into so that the core can use the data easier
pub struct cort{
    FCXO: String,
    v: i64,
    BLOCKXLINE: i64,
    LINES: i64,
    // prevmap is used so you dont render the same thing more than once saving some cpu usage
    prevmap: loader::map,
    
}
/*
PREVX: Vec<i64>,
    PREVY: Vec<i64>,
    PREVG: Vec<String>,
    */
// screen is the screen which stores the current map data
pub struct screen{
    pub chars: Vec<String>,
    pub x: Vec<i64>,
    pub y: Vec<i64>,
    pub delay: u64,
    
}
// used for the set up of cort 
impl core{
    pub fn setup(&self)->cort{

       
        self.setup001()
        

    }
    fn setup001(&self)-> cort{
        let xx = &self.name.to_string();
        let mut xxs = xx.to_string();
        xxs.push_str(&self.desc);
        xxs.push_str(" CCDB BANANA ALPHA");
        //println!("{}",xxs);
        cort{
            FCXO: xxs,
            v: 1,
            BLOCKXLINE: self.linelenght,
            LINES: self.lines,
            prevmap: loader::map{
                x: Vec::new(),
                y: Vec::new(),
                chars: Vec::new(),
            }
        }
    }
    
}

impl cort{
    fn equall(&self, map: loader::map)-> bool{

        if self.prevmap.x == map.x && self.prevmap.y == map.y && self.prevmap.chars == map.chars {
            return true
        }


        return false
    }
    pub fn render(&mut self, screen: &screen){

       // println!("■".green())
        
        
        thread::sleep(time::Duration::from_millis(screen.delay));

        if self.equall(screen.gmap()){

        }else{

            println!("{}",self.FCXO);
        
            self.prevmap = screen.run(self.LINES,self.BLOCKXLINE);

        }

    }
}
impl screen{

    
    pub fn run(&self, size: i64,size2: i64) -> loader::map{
       
        //parses the data correctly so that it gets outputed correctly
        for y in 0..size{
            for x in 0..size2{
                let mut charo = " ".to_string();
                for o in 0..self.chars.len(){
                    if x == self.x[o] && y == self.y[o]{
                        charo = self.chars[o].to_string();
                    }
                }
                print!("{}",charo);

            }
            println!("");
        }
       
        loader::map{
            x: self.x.clone(),
            y: self.y.clone(),
            chars: self.chars.clone(),
        }
    }
    
    pub fn loadmap(&mut self, map:loader::map){
        self.x = map.x;
        self.y = map.y;
        self.chars = map.chars;

    }
    fn gmap(&self)->loader::map{
        loader::map{
            x: self.x.clone(),
            y: self.y.clone(),
            chars: self.chars.clone()
        }
    }
}



