use crate::collision;
use crate::loader;

const BOUNCE: i64 = 1;
const GRAV: i64 = 1;
const CHANGE: f64 = 0.4;
const UPPOW: i64 = 1;

#[derive(Clone)]
pub struct droplet {
    pub pos: i64,
    pub vel: i64,
    pub forceDown: bool,
}
impl droplet {
    pub fn new(pos: i64) -> droplet {
        droplet {
            pos: pos,
            vel: 0,
            forceDown: false,
        }
    }
}

pub struct water {
    pub droplets: Vec<droplet>,
    pub chars: String,
}
impl water {
    pub fn collect_drop(&mut self, map: loader::map) {
        for x in map.find_all_of_X(self.chars.clone()) {
            self.droplets.push(droplet::new(x as i64));
        }
    }
    fn find_in_drop(&self, pos: i64) -> i64 {
        for x in 0..self.droplets.len() {
            if self.droplets[x].pos == pos {
                return x as i64;
            }
        }
        return -1;
    }
    pub fn react(&mut self, pos: i64) {
        let find_pos = self.find_in_drop(pos);
        if find_pos == -1 {
        } else {
            self.droplets[find_pos as usize].forceDown = true;
        }
    }

    pub fn run(&mut self, o: loader::map) -> loader::map {
        let mut screen = o;
        let mut fake_drop: Vec<droplet> = self.droplets.clone();

        for x in 0..self.droplets.len() {
            let mut side = false;
            let mut mat: droplet = fake_drop[x].clone();
            screen.y[mat.pos as usize] += 1;
            if !collision::get_collision(mat.pos as usize, &screen) {
                mat.vel += GRAV;
            } else {
                side = true;
            }
            screen.y[mat.pos as usize] -= 1;

            let vl = mat.vel;
            for p in 0..vl {
                screen.y[mat.pos as usize] += 1;
                let find = collision::find_collision(mat.pos as usize, &screen);
                if find != -1 {
                    screen.y[mat.pos as usize] -= 1;
                    let find_pos = self.find_in_drop(find);
                    if find_pos != -1 {
                        // fake_drop[find_pos as usize].vel = mat.vel;
                        if mat.vel >= 3 {
                            fake_drop[find_pos as usize].forceDown = true;
                        }

                        mat.vel = (mat.vel.clone() as f64 * CHANGE).floor() as i64;
                    }
                    //println!("{}",mat.forceDown);

                    break;
                }
            }

            //push it to the side or up if it cant go down
            if side {
                screen.x[mat.pos as usize] += 1;
                let pos = collision::find_collision(mat.pos as usize, &screen);

                if pos != -1 {
                    if mat.forceDown {
                        let find_pos = self.find_in_drop(pos);

                        if find_pos != -1 {
                            screen.y[pos as usize] -= UPPOW;
                            fake_drop[find_pos as usize].vel = 0;
                        }
                    }

                    screen.x[mat.pos as usize] -= 2;
                    let pos = collision::find_collision(mat.pos as usize, &screen);
                    if pos != -1 {
                        if mat.forceDown {
                            let find_pos = self.find_in_drop(pos);
                            if find_pos != -1 {
                                //screen.y[pos as usize] -= UPPOW;
                                fake_drop[find_pos as usize].vel = 0;
                            }
                            // mat.forceDown = false;

                            //x screen.y[mat.pos as usize] -= UPPOW;
                        } else {
                            screen.x[mat.pos as usize] += 1;
                        }
                    }
                }
                /*if mat.vel >= 1 {
                    for p in 0..mat.vel {
                        screen.y[mat.pos as usize] -= 1;
                        if collision::get_collision(mat.pos as usize, &screen) {
                            screen.y[mat.pos as usize] += 1;
                            break;
                        }
                    }
                    mat.vel = 0;
                }*/
            }

            fake_drop[x] = mat;
        }
        self.droplets = fake_drop;
        return screen;
    }
}
