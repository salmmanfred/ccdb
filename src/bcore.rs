//! Olive core
// Honestly would remove it if it wasent so damn good at its job some how 

use std::time::Duration;
use std::{thread, time};
use crate::loader;
use crate::physics;
use crate::sprite;


// core cort and screen is the same as in banana

pub struct core{
    pub name: String,
    pub desc: String,
    pub linelenght: i64,
    pub lines: i64,
    pub debug: bool,
    pub threads: i8,
    pub output_string: bool,

}
pub struct cort{
    FCXO: String,
    v: i64,
    pub BLOCKXLINE: i64,
    pub LINES: i64,
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

pub struct screen{
    pub chars: Vec<String>,
    pub x: Vec<i64>,
    pub y: Vec<i64>,
    pub delay: u64,
    pub sprite: Vec<sprite::sprite>,

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
            xxs.push_str(" CCDB BCORE");
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
            physobj: Vec::new(),
            debug: self.debug,
            gravity: 1,
            output_string: self.output_string,

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
    pub fn render(&mut self, screen: &mut screen) -> String{

       // println!("■".green())
        
        
        thread::sleep(time::Duration::from_millis(screen.delay));
        physics::Brenderphysics(screen, self.physobj.clone(),self.gravity);
        println!("{}",self.FCXO);

        if !self.equall(screen.gmap()){
            // sends it to the screen for render
        
            self.prevmap = screen.run(self.LINES,self.BLOCKXLINE, self);
        }
        if self.output_string == false{
            println!("{}",self.renderd);
        }else{
            return self.renderd.clone()

        }
        "".to_string()

    }
    fn prvrend(&mut self, f: String){ // for putting the previous render in the self.renderd
        self.renderd = f;
    }
    pub fn addphysics(&mut self,pos: i64){ // adds a object to be renderd for phycis later 
        self.physobj.push(pos);
    }
    pub fn addphysicsForAllX(&mut self,screen: &screen,chr: String){// adds allot of objects to be renderd for phycis later 
        for x in screen.findAllOfX(chr){
            self.physobj.push(x);
        }
    }
    pub fn removephysics(&mut self,pos: i64){ // removes physics
        self.physobj.retain(|&x| x != pos);
    }
    pub fn removephysicsForAllX(&mut self,screen: &screen,chr: String){// removes allot of objects to be renderd for phycis
        for x in screen.findAllOfX(chr){
            self.removephysics(x);
        }
    }
    pub fn changePhysics(&mut self, grav: i64){ // change the physics to be what ever you want it to be 
        self.gravity = grav;
    }
}
impl screen{
    pub fn run(&self, size: i64,size2: i64, cort: &mut cort) -> loader::map{
        let mut line: String = "".to_string();
        let mut X = self.x.clone();// load in a sprite
        let mut Y = self.y.clone();
        let mut C = self.chars.clone();

        for x in 0..self.sprite.len(){// does the actually loading 
            C.extend(self.sprite[x].chars.clone());
            X.extend(self.sprite[x].x.clone());
            Y.extend(self.sprite[x].y.clone());
        }

        for sx in 0..size{// splits everything into a bunch of X lines then calls make line to make said x lines 
            let mut betterx:Vec<i64> = Vec::new();
            let mut bettern:Vec<String> = Vec::new();
            //betterx.remove()
            // splits it into the correct y section
            for x in 0..C.len(){
                if sx == Y[x]{
                    betterx.push(X[x]);
                    bettern.push(C[x].clone());
                    
                }

            }
           /* for x in 0..self.sprite.len(){
                if self.sprite[x].y[x] == sx{
                }
            }*/
            // makes the line and prints it 
            line.push_str(&format!("{}\n",&self.makeline(betterx,bettern,size2)));

        }
        cort.prvrend(line);
        loader::map{
            x: X.clone(),
            y: Y.clone(),
            chars: C.clone(),
        }
    }
    pub fn makeline(&self, betterx: Vec<i64>,bettern: Vec<String>,size:i64) -> String{
        let mut vc: Vec<&str> = Vec::new();
        // really complicated version of what was in banana
        for i in 0..size{// size is the lenght of the entire line
            let masi = i;
            let mut rus = 1;
            let mut run = true;
            for i in 0..betterx.len(){// finds the correct char? 
                
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
    pub fn gmap(&self)->loader::map{ // makes a map out of the current data in screen
        
        let mut charss = self.chars.clone();
        let mut xxx = self.x.clone();
        let mut yyy = self.y.clone();

        for x in 0..self.sprite.len(){//appends the sprite 
            charss.extend(self.sprite[x].chars.clone());
            xxx.extend(self.sprite[x].x.clone());
            yyy.extend(self.sprite[x].y.clone());

        }
         loader::map{
            x: xxx,
            y: yyy,
            chars: charss
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



