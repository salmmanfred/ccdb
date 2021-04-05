
use std::time::Duration;
use std::{thread, time};
use crate::loader;
use crate::colour;
use crate::check::{threadCheck};
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
    pub delay: i64,
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
    stb: i64,
    
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
        if self.debug{
            xxs.push_str(" CCDB BANANA ALPHA");

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
            stb: self.delay,
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

        println!("{}",self.FCXO);
        
        self.prevmap = screen.run(self.LINES,self.BLOCKXLINE,self.thr,self.stb);
        if self.equall(screen.gmap()){

        }else{

           

        }

    }
}
impl screen{

    
    pub fn run(&self, size: i64,size2: i64,thr:i8,stb:i64) -> loader::map{
        // = Vec::with_capacity(10);
        //parses the data correctly so that it gets outputed correctly
        let mut aot = 0;
        let mut sso = 0;
        
            //let (tx, rx) = mpsc::channel();
            aot += 1;
            

                let mut hands: Vec<std::thread::JoinHandle<()>> = Vec::new();


                let mut prev = 0;

        
        for P in 0..thr as i64{
            sso += 1;
            let chars = self.chars.clone();
            let xx = self.x.clone();
            let yy = self.y.clone();
            let mut chunky1 = 0;
            let mut tsize = size;
            let stb2 = stb;
            if size % 2 != 0{
                tsize += 1;
            }
            if P > 0{
                
                chunky1 = prev+tsize/thr as i64;
                prev = chunky1;
            }
            //let prvthr = &prvth.to_owned();
                

            
            let chunky2 = chunky1+tsize/thr as i64;


          
                  
            hands.push(thread::spawn( move|| {
                
                let mut fchunk = "".to_string();
                for y in chunky1..chunky2{
                let mut row = "".to_string();
                    for x in 0..size2{
                        let mut charo = " ".to_string();
                        for o in 0..chars.len(){
                            if x == xx[o] && y == yy[o]{
                                charo = chars[o].to_string();
                            }
                        }
                        row.push_str(&format!("{}",charo));
                    
                    }
                fchunk.push_str(&format!("{}\n",row));
                }
                if P >= 1{
                thread::sleep(time::Duration::from_nanos(stb2 as u64));
               
                }
                print!("{}",fchunk);
                   
            }));
            
        }
                
            //}
                //hands.push(thr);
        for thr in hands{
            thr.join().unwrap();  
        } 
            

            
            //println!("");
        
       
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



