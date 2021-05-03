use crate::core::{Cort, Screen};
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

pub fn run(screen: &Screen, Cort: &mut Cort) -> loader::map {
    let now = Instant::now();

    // = Vec::with_capacity(10);
    //parses the data correctly so that it gets outputed correctly
    //getting the correct data and gets how many threads we need and the y size and x size
    let thr = Cort.thr;
    let size = Cort.lines;
    let size2 = Cort.char_x_line;

    let mut aot = 0;
    let mut sso = 0;

    //let (tx, rx) = mpsc::channel();
    aot += 1;

    let mut hands: Vec<std::thread::JoinHandle<()>> = Vec::new(); //stores the threads

    let mut prev = 0;
    let mut charss = screen.chars.clone();
    let mut xxx = screen.x.clone();
    let mut yyy = screen.y.clone();

    for x in 0..screen.sprite.len() {
        //appends the sprite
        charss.extend(screen.sprite[x].chars.clone());
        xxx.extend(screen.sprite[x].x.clone());
        yyy.extend(screen.sprite[x].y.clone());
    }

    let mut megadata: Vec<Arc<Mutex<String>>> = Vec::new(); // collect all the mutexes in the same vector
    for x in 0..thr {
        megadata.push(Arc::new(Mutex::new("".to_string())));
    }

    let (tx, rx) = channel();

    for P in 0..thr as i64 {
        let (data, tx) = (Arc::clone(&megadata[P as usize]), tx.clone()); //get the correct mutex

        sso += 1; // creates all the variables since they are move they need to be re defined
        let threadsize = thr;

        let mut chars = charss.clone();
        let mut xx = xxx.clone();
        let mut yy = yyy.clone();
        let mut chunk1 = 0;
        let mut tsize = size;
        if size % 2 != 0 {
            tsize += 1;
        }
        if P > 0 {
            // calculates the chunk

            chunk1 = prev + tsize / thr as i64;
            prev = chunk1;
        }
        let chunk2 = chunk1 + tsize / thr as i64;

        hands.push(thread::spawn(move || {
            // creates a thread to start working on a chunk
            //let now = Instant::now();

            let mut fchunk = "".to_string();
            let mut smallest = 0;
            let mut biggest = 0;
            let mut open = true;
            for i in 0..chars.len() {
                // get the smallest index for finding all the chars that the threads need and the biggest
                if yy[i] >= chunk1 && open {
                    smallest = i;
                    open = false;
                }
                if yy[i] <= chunk2 {
                    biggest = i + 1;
                }
            }

            //println!(" po{},{}",biggest,chars.len());

            //println!("f{},{}",smallest,biggest);

            for y in chunk1..chunk2 {
                // this is where the magic happens
                let mut row = "".to_string();
                let mut vectorY: Vec<String> = Vec::new();
                let mut vectorX: Vec<i64> = Vec::new();

                for i in smallest..biggest {
                    // get the correct data for the Y
                    if yy[i] == y {
                        vectorY.push(chars[i].clone());
                        vectorX.push(xx[i]);
                    }
                }
                let mut donesofar = 0;
                for x in 0..size2 {
                    let mut ch = " ";
                    if donesofar <= vectorY.len() {
                        //println!("so{},{}",chars.len(),vectorY.len());
                        /* for o in vectorY.iter(){
                            let o = o.to_owned() as usize;
                            if x == xx[o]{
                                donesofar += 1;
                                ch = &chars[o];
                                break;

                            }
                        }*/
                        for i in 0..vectorX.len() {
                            // finds the correct char?

                            if vectorX[i] == x {
                                donesofar += 1;
                                ch = &vectorY[i];

                                break;
                            }
                        }
                    } else {
                        //break;
                    }

                    row.push_str(&ch); // push it together to a single line
                }

                fchunk.push_str(&format!("{}\n", row));
            }

            // push back the data
            let mut data = data.lock().unwrap(); // get data
            data.push_str(&fchunk); //push the data
            drop(data);

            //send data back
            tx.send(()).unwrap();

            //println!("{}    ",now.elapsed().as_millis());
        }));
    }

    //}
    //hands.push(thr);

    for thr in hands {
        thr.join().unwrap(); // join all threads
    }
    let finals = rx.recv().unwrap();

    let mut fdata = "".to_string();
    for x in megadata {
        fdata.push_str(&x.lock().unwrap());
    }

    if !Cort.output_string {
        // checks if it should print it out or not
        println!("{}", fdata);
    }

    //println!("");
    Cort.renderd = fdata;
    Cort.extime = now.elapsed().as_millis() as i64;

    screen.gmap() // returns the current map of Screen to be put in prevmap.
}
