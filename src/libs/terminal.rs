use crate::escape;
use crate::check::check_win;

pub fn get_terminal_size() -> [usize; 2] {
    let x: [usize; 2];

    unsafe {
        //x = [getwinsizeCOL(), getwinsizeROW()];
    }
    panic!("this feature has been removed");
    return x;
}

fn set_terminal_name(name: &str){
   // println!("\x1b]30;{}\a",name);
   panic!("error")
}
pub fn set_terminal_size(x: usize, y: usize) {
    check_win();
    print!("\x1B[8;{};{}t",y,x);
    
}

pub fn restore(){
        escape::restore();
}
pub fn save(){
    escape::restore();
}