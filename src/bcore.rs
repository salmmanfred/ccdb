//! Olive core
use std::time::Duration;
use std::{thread, time};
use crate::loader;

// core cort and screen is the same as in banana

pub struct core{
    pub name: String,
    pub desc: String,
    pub linelenght: i64,
    pub lines: i64,
    pub debug: bool,
    pub threads: i8,
    pub delay: i64,
}
pub struct cort{
    FCXO: String,
    v: i64,
    BLOCKXLINE: i64,
    LINES: i64,
    prevmap: loader::map,
    renderd: String,
    
}
/*
PREVX: Vec<i64>,
    PREVY: Vec<i64>,
    PREVG: Vec<String>,
    */

pub struct screen{
    pub chars: Vec<String>,
    pub x: Vec<i64>,
    pub y: Vec<i64>,
    pub delay: u64,
}
impl core{
    pub fn setup(&self)->cort{

       
        self.setup001()
        

    }
    fn setup001(&self)-> cort{
        let xx = &self.name.to_string();
        let mut xxs = xx.to_string();
        xxs.push_str(&self.desc);
        if self.debug{
            xxs.push_str(" CCDB BCORE ALPHA");
        }
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
            },
            renderd: "".to_string(),
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
    pub fn render(&mut self, screen: &screen) -> String{

       // println!("■".green())
        
        
        thread::sleep(time::Duration::from_millis(screen.delay));

        if self.equall(screen.gmap()){

        }else{
            // sends it to the screen for render
            println!("{}",self.FCXO);
        
            self.prevmap = screen.run(self.LINES,self.BLOCKXLINE, self);

        }
         self.renderd.clone()

    }
    fn prvrend(&mut self, f: String){
        self.renderd = f;
    }
}
impl screen{
    pub fn run(&self, size: i64,size2: i64, cort: &mut cort) -> loader::map{
        let mut line: String = "".to_string();

        for sx in 0..size{
            let mut betterx:Vec<i64> = Vec::new();
            let mut bettern:Vec<String> = Vec::new();
            //betterx.remove()
            // splits it into the correct y section
            for x in 0..self.chars.len(){
                if sx == self.y[x]{
                    betterx.push(self.x[x]);
                    bettern.push(self.chars[x].clone());
                    
                }

            }
            // makes the line and prints it 
            line.push_str(&format!("{}\n",&self.makeline(betterx,bettern,size2)));

        }
        cort.prvrend(line);
        loader::map{
            x: self.x.clone(),
            y: self.y.clone(),
            chars: self.chars.clone(),
        }
    }
    pub fn makeline(&self, betterx: Vec<i64>,bettern: Vec<String>,size:i64) -> String{
        let mut vc: Vec<&str> = Vec::new();
        // really complicated version of what was in banana
        for i in 0..size{
            let masi = i;
            let mut rus = 1;
            let mut run = true;
            for i in 0..betterx.len(){
                
                    if betterx[i] == masi{
                        vc.push(&bettern[i]);
                        rus = 0;
                        break;
                    }

            }
            if run{
                if rus == 1{
                    vc.push(" ");
        
                }
                if i == 0{
                    vc.push("");
        
                }/*else if i == size-1{
        
                    vc.push("");
                    break;
        
                }*/
            }
               
    
    
        }
        vc.into_iter().collect::<String>()

    }
    pub fn loadmap(&mut self, map:loader::map){
        // loads the map into the screen
        self.x = map.x;
        self.y = map.y;
        self.chars = map.chars;

    }
    pub fn gmap(&self)->loader::map{
        // makes the data in the screen into a map
        loader::map{
            x: self.x.clone(),
            y: self.y.clone(),
            chars: self.chars.clone()
        }
    }
    pub fn findX(&self, ch: String) -> i64{
        for x in 0..self.chars.len(){
            if self.chars[x] == ch{
                return x as i64
            }
        }
        0

    }
    pub fn findAllOfX(&self, ch: String) -> Vec<i64>{
        let mut all: Vec<i64> = Vec::new();
        for x in 0..self.chars.len(){
            if self.chars[x] == ch{
                all.push(x as i64);
            }
        }
        all
    }
}



