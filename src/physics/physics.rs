use crate::acore;
use crate::bcore;
use crate::collision;
use crate::loader::map;
//const Gdrag: i64 = 1;

pub fn a_render_physics(screen: &mut acore::Screen, physobj: Vec<i64>, G: i64) {
    // since acore and bcore use diffrent screens i have to do it this way
    let map = screen.gmap();
    let phsob = physobj;
    screen.load_map(renderphys(map, phsob, G));
}
pub fn b_render_physics(screen: &mut bcore::Screen, physobj: Vec<i64>, G: i64) {
    let map = screen.gmap();
    let phsob = physobj;
    screen.load_map(renderphys(map, phsob, G));
}

fn renderphys(screen: map, physobj: Vec<i64>, g_drag: i64) -> map {
    let mut mp = map {
        chars: screen.chars,
        x: screen.x,
        y: screen.y,
    };

    for i in physobj {
        // just adds physics to all objects
        for _ in 0..g_drag {
            mp.y[i as usize] += 1;
            if collision::get_collision(i as usize, &mut mp) {
                mp.y[i as usize] -= 1;
            }
        }
    }
    mp
}
