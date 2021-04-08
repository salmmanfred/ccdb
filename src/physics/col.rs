use crate::loader;
/*
struct colmap{
    pub cols: Vec<i64>,
    pub with: Vec<i64>
}*/


pub fn getcol(pos: usize,screen: &mut loader::map)->bool{ //just sees if an object has collided with another
    

    for x in 0..screen.chars.len(){
        if x != pos{
            if screen.x[x] == screen.x[pos]{
                if screen.y[x] == screen.y[pos]{
                    return true
                }
            }
        }
        
    }
    false

    
}