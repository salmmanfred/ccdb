//use crate::loader;

pub struct sprite {
    pub chars: Vec<String>,
    pub x: Vec<i64>,
    pub y: Vec<i64>,
}
impl sprite {
    pub fn move_xy(&mut self, modif_x: i64, modif_y: i64) {
        for x in 0..self.x.len() {
            self.x[x] += modif_x;
            self.y[x] += modif_y;
        }
    }
    pub fn set_xy(&mut self, x: i64, y: i64) {
        let difx = self.x[0] - x;
        let dify = self.y[0] - y;
        for x in 0..self.x.len() {
            self.x[x] += difx;
            self.y[x] += dify;
        }
    }
}

pub fn load(filename: &str) -> sprite {
    let file = openfile::readFileLines(filename); // calls and returns what loadvec does
    loadvec(file)
}

pub fn loadvec(file: Vec<String>) -> sprite {
    let mut x: Vec<i64> = Vec::new();
    let mut y: Vec<i64> = Vec::new();
    let mut c: Vec<String> = Vec::new();
    let mut yy = 0;
    let mut xx = 0;

    for i in file {
        for ii in i.chars() {
            // ignore the spaces
            if ii != ' ' {
                x.push(xx);
                y.push(yy);
                c.push(ii.to_string())
            }
            xx += 1;
        }
        xx = 0;
        yy += 1;
    }

    sprite {
        chars: c,
        x: x,
        y: y,
    }
}
