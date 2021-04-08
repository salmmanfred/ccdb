
use crate::acore;
use crate::bcore;
use crate::col;
use crate::loader::{map};
//const Gdrag: i64 = 1;

pub fn Arenderphys(screen: &mut acore::screen,physobj: Vec<i64>, G: i64){ // since acore and bcore use diffrent screens i have to do it this way
    let mut map = screen.gmap();
    let phsob = physobj;
    screen.loadmap(renderphys(map, phsob,G));
}
pub fn Brenderphys(screen: &mut bcore::screen,physobj: Vec<i64>, G: i64){
    let mut map = screen.gmap();
    let phsob = physobj;
    screen.loadmap(renderphys(map, phsob,G));
}


fn renderphys(screen: map,physobj: Vec<i64>,Gdrag: i64)->map{
    let mut mp = map{
        chars: screen.chars,
        x: screen.x,
        y: screen.y,

    };
    
    for i in physobj{
        mp.y[i as usize] += Gdrag;
        if col::getcol(i as usize, &mut mp){
            mp.y[i as usize] -= Gdrag;

        }
    }   
    mp
}

