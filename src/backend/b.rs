use crate::core::{Cort, Screen};
use crate::loader;
use std::time::Duration;
use std::{thread, time};
//use crate::colour;
use crate::physics;
use crate::sprite;
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
use std::time::Instant;

pub fn run(screen: &Screen, size: i64, size2: i64, Cort: &mut Cort) -> loader::map {
    let mut line: String = "".to_string();
    let mut X = screen.x.clone(); // load in a sprite
    let mut Y = screen.y.clone();
    let mut C = screen.chars.clone();

    for x in 0..screen.sprite.len() {
        // does the actually loading
        C.extend(screen.sprite[x].chars.clone());
        X.extend(screen.sprite[x].x.clone());
        Y.extend(screen.sprite[x].y.clone());
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
        /* for x in 0..screen.sprite.len(){
            if screen.sprite[x].y[x] == sx{
            }
        }*/
        // makes the line and prints it
        line.push_str(&format!(
            "{}\n",
            make_line(&screen, betterx, bettern, size2)
        ));
    }
    Cort.renderd = line;
    loader::map {
        x: X.clone(),
        y: Y.clone(),
        chars: C.clone(),
    }
}
pub fn make_line(screen: &Screen, betterx: Vec<i64>, bettern: Vec<String>, size: i64) -> String {
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
