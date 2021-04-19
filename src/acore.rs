//! Banana core 
// ok sure this is slower but damn its much more fun to work on this 
use std::time::Duration;
use std::{thread, time};
use crate::loader;
use crate::colour;
use crate::check::{threadCheck};
use crate::physics;
use crate::sprite;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::channel;
use std::time::{Instant};

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
    pub BLOCKXLINE: i64,
    pub LINES: i64,
    // prevmap is used so you dont render the same thing more than once saving some cpu usage
    prevmap: loader::map,
    thr: i8,
    physobj: Vec<i64>,
    gravity: i64,
    debug: bool,
    renderd: String,
    output_string: bool,
    pub extime: i64,
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
            xxs.push_str(" CCDB ACORE");

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
            extime: 0,
        }
    }
    
}



impl cort{
    fn equall(&self, map: loader::map)-> bool{

        if self.prevmap.x == map.x  && self.prevmap.y == map.y && self.prevmap.chars == map.chars{

            return true  // checks if the screen and a map is the same 

            
        }
        



        return false
    }
    pub fn render(&mut self, screen: &mut screen)-> String{

       // ■
        
        
        thread::sleep(time::Duration::from_millis(screen.delay));

        println!("{}",self.FCXO);
        physics::Arenderphysics(screen, self.physobj.clone(),self.gravity);
        if !self.equall(screen.gmap()){ // helps preformance by not rendering the same window again and again and again
           
            
            self.prevmap = screen.run(self);  // starts the screen rendering 
        }else{
            if !self.output_string{
                println!("{}",self.renderd)
            }
        }

        if self.output_string{
            return self.renderd.clone()
        }else{
            return "".to_string()
        }
        

    }
    pub fn addphysics(&mut self,pos: i64){// adds a object to be renderd for phycis later 
        self.physobj.push(pos);
    }
    pub fn addphysicsForAllX(&mut self,screen: &screen,chr: String){// adds allot of objects to be renderd for phycis later 
        for x in screen.findAllOfX(chr){
            self.physobj.push(x);
        }
    }
    pub fn removephysics(&mut self,pos: i64){ // removes physics from object
        self.physobj.retain(|&x| x != pos);
    }
    pub fn removephysicsForAllX(&mut self,screen: &screen,chr: String){// removes allot of objects to be renderd for phycis
        for x in screen.findAllOfX(chr){
            self.removephysics(x);
        }
    }
    pub fn changePhysics(&mut self, grav: i64){ 
        self.gravity = grav;
    }
    /*pub fn changeLINES(&mut self,newlines: i64){
        self.LINES = newlines;
    }
    pub fn changeXperLine(&mut self,x: i64){
        self.BLOCKXLINE = x;
    }*/

}
impl screen{

    
    pub fn run(&self,cort:&mut  cort) -> loader::map{
        let now = Instant::now();

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
        
        
        
        let mut megadata: Vec<Arc<Mutex<String>>> = Vec::new();// collect all the mutexes in the same vector
        for x in 0..thr{
            megadata.push(Arc::new(Mutex::new("".to_string())));
        }
       

        let (tx, rx) = channel();

        for P in 0..thr as i64{
            let (data, tx) = (Arc::clone(&megadata[P as usize]), tx.clone());//get the correct mutex 
            /*
            let (data, tx) = (Arc::clone(&data), tx.clone());
            let (current, tx2) = (Arc::clone(&current), tx.clone());
            let (data2) = (Arc::clone(&data2));
            let (current2) = (Arc::clone(&current2));*/
            

            sso += 1;// creates all the variables since they are move they need to be re defined
            let threadsize = thr;
          
            let mut chars = charss.clone();
            let mut xx = xxx.clone();
            let mut yy = yyy.clone();
            let mut chunk1 = 0;
            let mut tsize = size;
            if size % 2 != 0{
                tsize += 1;
            }
            if P > 0{                           // calculates the chunk 
                
                chunk1 = prev+tsize/thr as i64;
                prev = chunk1;
            }
            let chunk2 = chunk1+tsize/thr as i64;


          
                
            hands.push(thread::spawn( move|| { // creates a thread to start working on a chunk
                //let now = Instant::now();
            
                let mut fchunk = "".to_string();
                let mut smallest = 0;
                let mut biggest = 0;
                let mut open = true;
                for i in 0..chars.len(){// get the smallest index for finding all the chars that the threads need and the biggest
                    if yy[i] >= chunk1 && open{
                        smallest = i;
                        open = false;
                    }
                    if yy[i] <= chunk2{
                        biggest = i+1;
                    }
                    
                }

               
                //println!(" po{},{}",biggest,chars.len());

                //println!("f{},{}",smallest,biggest);
                


                for y in chunk1..chunk2{// this is where the magic happens
                    let mut row = "".to_string();
                    let mut vectorY: Vec<String> = Vec::new();
                    let mut vectorX: Vec<i64> = Vec::new();


                    for i in smallest..biggest{// get the correct data for the Y 
                        if yy[i] == y{
                            vectorY.push(chars[i].clone());
                            vectorX.push(xx[i]);

                            
                        }
                    }
                    let mut donesofar = 0;
                    for x in 0..size2{
                       
                        let mut ch = " ";
                        if donesofar <= vectorY.len(){
                            //println!("so{},{}",chars.len(),vectorY.len());
                           /* for o in vectorY.iter(){
                                let o = o.to_owned() as usize;
                                if x == xx[o]{
                                    donesofar += 1;
                                    ch = &chars[o];
                                    break;

                                } 
                            }*/
                            for i in 0..vectorX.len(){// finds the correct char? 
                
                                if vectorX[i] == x{
                                    donesofar +=1;
                                    ch = &vectorY[i];
                                    
                                    break;
                                }
            
                            }
                        }else{
                            //break;
                        }


                        row.push_str(&ch);  // push it together to a single line
                    
                    }


                    fchunk.push_str(&format!("{}\n",row));
                }

                // push back the data
                let mut data = data.lock().unwrap();// get data
                data.push_str(&fchunk);//push the data
                drop(data);
                
               

                


                

                //send data back
                tx.send(()).unwrap();
                   

                //println!("{}    ",now.elapsed().as_millis()); 

                
            }));
            
        }
                
            //}
                //hands.push(thr);

        for thr in hands{
            thr.join().unwrap();  // join all threads
        }
        let finals = rx.recv().unwrap();

        

        let mut fdata = "".to_string();
        for x in megadata{
            fdata.push_str(&x.lock().unwrap());
        }

        if !cort.output_string{// checks if it should print it out or not
            println!("{}",fdata);
        }
            
            //println!("");
        cort.renderd = fdata;
        cort.extime = now.elapsed().as_millis() as i64; 
        
        
        self.gmap()// returns the current map of screen to be put in prevmap.
        
    }


    
    pub fn loadmap(&mut self, map:loader::map){ // for loading a map into the screen
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



