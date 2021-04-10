//! Banana core 
use std::time::Duration;
use std::{thread, time};
use crate::loader;
use crate::colour;
use crate::check::{threadCheck};
use crate::physics;
use crate::sprite;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::channel;
//use std::sync::{Mutex, Arc};
//use crate::messages::{message,listen};
//this is the core used for things like declaring the line lenght and amount of lines
pub struct core{
    pub name: String,
    pub desc: String,
    pub linelenght: i64,
    pub lines: i64,
    pub debug: bool,
    pub threads: i8,
    pub output_string: bool,
}
// this is what core "compiles" into so that the core can use the data easier
pub struct cort{
    FCXO: String,
    v: i64,
    BLOCKXLINE: i64,
    LINES: i64,
    // prevmap is used so you dont render the same thing more than once saving some cpu usage
    prevmap: loader::map,
    thr: i8,
    physobj: Vec<i64>,
    gravity: i64,
    debug: bool,
    renderd: String,
    output_string: bool,
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
    pub sprite: Vec<sprite::sprite>,
    
}
// used for the set up of cort 
impl core{
    pub fn setup(&self)->cort{

       
        self.setup001()
        

    }
    fn setup001(&self)-> cort{ // compiles core into cort where data and meta data is stored 
        let xx = &self.name.to_string();
        let mut xxs = xx.to_string();
        xxs.push_str(&self.desc);
        if self.debug{
            xxs.push_str(" CCDB ACORE ALPHA");

        }
        //println!("{}",xxs);
        threadCheck(self.threads, self.lines);
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
            thr: self.threads,
            physobj: Vec::new(),
            debug: self.debug,
            gravity: 1,
            renderd: "".to_string(),
            output_string: self.output_string,
        }
    }
    
}

impl cort{
    fn equall(&self, map: loader::map)-> bool{

        if self.prevmap.x == map.x && self.prevmap.y == map.y && self.prevmap.chars == map.chars {
            return true  // checks if the screen and a map is the same 
        }


        return false
    }
    pub fn render(&mut self, screen: &mut screen)-> String{

       // println!("■".green())
        
        
        thread::sleep(time::Duration::from_millis(screen.delay));

        println!("{}",self.FCXO);
        physics::Arenderphys(screen, self.physobj.clone(),self.gravity);
        if !self.equall(screen.gmap()){ // helps preformance by not rendering the same window again and again and again
            
            self.prevmap = screen.run(self);  // starts the screen rendering 

        }
        if self.output_string{
            return self.renderd.clone()
        }else{
            return "".to_string()
        }
        

    }
    pub fn addphys(&mut self,pos: i64){// adds a object to be renderd for phycis later 
        self.physobj.push(pos);
    }
    pub fn addphysForAllX(&mut self,screen: &screen,chr: String){// adds allot of objects to be renderd for phycis later 
        for x in screen.findAllOfX(chr){
            self.physobj.push(x);
        }
    }
    pub fn removephys(&mut self,pos: i64){ // removes physics from object
        self.physobj.retain(|&x| x != pos);
    }
    pub fn removephysForAllX(&mut self,screen: &screen,chr: String){// removes allot of objects to be renderd for phycis
        for x in screen.findAllOfX(chr){
            self.removephys(x);
        }
    }
    pub fn changePhysics(&mut self, grav: i64){
        self.gravity = grav;
    }

}
impl screen{

    
    pub fn run(&self,cort:&mut  cort) -> loader::map{
        // = Vec::with_capacity(10);
        //parses the data correctly so that it gets outputed correctly
        let thr = cort.thr;
        let size = cort.LINES;
        let size2 = cort.BLOCKXLINE;
        
        let mut aot = 0;
        let mut sso = 0;
        
            //let (tx, rx) = mpsc::channel();
            aot += 1;
            

        let mut hands: Vec<std::thread::JoinHandle<()>> = Vec::new();  //stores the threads


        let mut prev = 0;
        let mut charss = self.chars.clone();
        let mut xxx = self.x.clone();
        let mut yyy = self.y.clone();
        
        for x in 0..self.sprite.len(){//appends the sprite 
            charss.extend(self.sprite[x].chars.clone());
            xxx.extend(self.sprite[x].x.clone());
            yyy.extend(self.sprite[x].y.clone());

        }
        
        let data = Arc::new(Mutex::new("".to_string()));
        let current = Arc::new(Mutex::new(0 as i64));

        let (tx, rx) = channel();

        for P in 0..thr as i64{
            let (data, tx) = (Arc::clone(&data), tx.clone());
            let (current, tx2) = (Arc::clone(&current), tx.clone());


            sso += 1;// creates all the variables since they are move they need to be re defined
            let threadsize = thr;
            let mut chars = charss.clone();
            let mut xx = xxx.clone();
            let mut yy = yyy.clone();
            let mut chunky1 = 0;
            let mut tsize = size;
            if size % 2 != 0{
                tsize += 1;
            }
            if P > 0{                           // calculates the chunk 
                
                chunky1 = prev+tsize/thr as i64;
                prev = chunky1;
            }
            let chunky2 = chunky1+tsize/thr as i64;


          
                
            hands.push(thread::spawn( move|| { // creates a thread to start working on a chunk
                
                let mut fchunk = "".to_string();
                for y in chunky1..chunky2{// this is where the magic happens
                let mut row = "".to_string();
                    for x in 0..size2{
                        let mut charo = " ".to_string();
                        for o in 0..chars.len(){
                            if x == xx[o] && y == yy[o]{
                                charo = chars[o].to_string();
                            }
                        }
                        row.push_str(&format!("{}",charo));  // push it together to a single line
                    
                    }
                fchunk.push_str(&format!("{}\n",row));
                }
                /*if P >= 1{
                    thread::sleep(time::Duration::from_nanos(stb2 as u64)); // delay so that the chunks will be printed in the correct oder 
                    
                }*/

                loop{// this checks if it is its turn to push the data
                    let mut current = current.lock().unwrap();
                    //println!("waiting...{},{}",P,*current);
                    if *current == P{
                        *current += 1;
                        let mut data = data.lock().unwrap();
                        //print!("{}",fchunk);
                        data.push_str(&fchunk);
                        drop(data);
                        drop(current);
                        break;
                    }

                }
                /*println!("current{}",current);

                *current += 1;
                drop(current);*/


                

                //last thread sends the data back
                if P == threadsize as i64 - 1{
                    tx.send(()).unwrap();
                    tx2.send(()).unwrap();

                   // println!("\ndone{}{}",P,threadsize);
                
                }
                
            }));
            
        }
                
            //}
                //hands.push(thr);

        for thr in hands{
            thr.join().unwrap();  // join all threads
        } 
        let finals = rx.recv().unwrap();

        let newdata = data.lock().unwrap();// data

        if !cort.output_string{// checks if it should print it out or not
            println!("{}",newdata);
        }
            
            //println!("");
        cort.renderd = format!("{}",newdata);
        
       
        loader::map{// returns the current map of screen to be put in prevmap.
            x: self.y.clone(),
            y: self.y.clone(),
            chars: self.chars.clone(),
        }
    }


    
    pub fn loadmap(&mut self, map:loader::map){ // for loading a map into the screen
        self.x = map.x;
        self.y = map.y;
        self.chars = map.chars;

    }
    pub fn gmap(&self)->loader::map{ // makes a map out of the current data in screen
        loader::map{
            x: self.x.clone(),
            y: self.y.clone(),
            chars: self.chars.clone()
        }
    }
    pub fn findX(&self, ch: String) -> i64{ // for finding a character 
        for x in 0..self.chars.len(){
            if self.chars[x] == ch{
                return x as i64
            }
        }
        0

    }
    pub fn findAllOfX(&self, ch: String) -> Vec<i64>{ // returns a vector of the position of all instanses of a certain character
        let mut all: Vec<i64> = Vec::new();
        for x in 0..self.chars.len(){
            if self.chars[x] == ch{
                all.push(x as i64);
            }
        }
        all
    }

    
}



