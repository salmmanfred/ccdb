use crate::keyin;

pub enum ButtonAction {
    Press,
    Idle,
    Hover,
}

pub struct Button {
    pub text: String,
    pub pos: i64,
    imsel: bool, // if the button is pressed down
    pub hover: bool,
}
impl Button {
    pub fn new(y: i64, text: &str) -> Button {
        Button {
            text: text.to_string(),
            pos: y,
            imsel: false,
            hover: false,
        }
    }
    pub fn get_status(&mut self) -> ButtonAction {
        //gets the status

        if self.imsel {
            self.imsel = false;
            return ButtonAction::Press;
        }

        if self.hover {
            self.hover = false;

            return ButtonAction::Hover;
        }
        return ButtonAction::Idle;
    }
}

pub struct UI {
    pub buttons: Vec<Button>,
    size: [i64; 2],
    //controls: [usize; 3],
    index: i64,
    ent: bool, // if enter is pressed
               // mt: i64,
}
impl UI {
    pub fn new() -> UI {
        UI {
            buttons: Vec::new(),
            size: [100, 100],
            //controls: [up,down,select],
            index: 0,
            ent: false,
            // mt: 0,
        }
    }
    pub fn ups(&mut self) {
        // get the keyboard input stuff

        //if keyin::key_down(){

        match keyin::get_key() {
            72 => {
                self.index -= 1;
            }
            80 => {
                self.index += 1;
            }

            13 => {
                self.ent = true;
                //break;
            }
            _ => {}
        }
        //}
    }
    pub fn rend(&mut self) {
        self.rend_backend();
    }
    pub fn rend_backend(&mut self) {
        // render it all
        self.ent = false;

        let mut i = 0;
        //let mut sele = false;
        for _ in 0..10 {
            // stupid solution to a stupid issue
            self.ups();
        }

        if self.index < 0 {
            // make sure its not outside of the buttons
            self.index = 0;
        }
        if self.index >= self.buttons.len() as i64 - 1 {
            self.index = self.buttons.len() as i64 - 1;
        }

        // println!("i{}",self.index);
        //println!("");

        for y in 0..self.size[1] {
            // makes the ui stuff
            let mut ch = " ".to_string();
            if i > self.buttons.len() {
                break;
            }
            if y == self.index {
                ch = ">".to_string();
            }
            for o in self.buttons.iter() {
                if o.pos == y {
                    println!("{}|{}", ch, o.text);
                    i += 1;

                    break;
                }
            }
        }

        for x in 0..self.buttons.len() {
            // gets the button that is hoverd or pressed
            if self.buttons[x].pos == self.index {
                if self.ent {
                    self.buttons[x].imsel = true;
                } else {
                    self.buttons[x].hover = true;
                }
            } else {
                self.buttons[x].imsel = false;
                self.buttons[x].hover = false;
            }
        }
    }
}

/*pub fn makebutton(x: i64,y: i64,text: &str)-> button{

}*/
/*fn rendT() {
    unsafe {
        GetPosCurX();
        GetPosCurY();

        //println!("o{},{}                 ",,GetPosCurY());
    }
}*/
