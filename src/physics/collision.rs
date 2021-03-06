use crate::loader;
/*
struct colmap{
    pub cols: Vec<i64>,
    pub with: Vec<i64>
}*/

pub fn get_collision(pos: usize, screen: &loader::map) -> bool {
    //just sees if an object has collided with another

    for x in 0..screen.chars.len() {
        if x != pos {
            if screen.x[x] == screen.x[pos] {
                if screen.y[x] == screen.y[pos] {
                    return true;
                }
            }
        }
    }
    false
}

pub fn find_collision(pos: usize, screen: &loader::map) -> i64 {
    for x in 0..screen.chars.len() {
        if x != pos {
            if screen.x[x] == screen.x[pos] {
                if screen.y[x] == screen.y[pos] {
                    return x as i64;
                }
            }
        }
    }
    return -1;
}
