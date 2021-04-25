use crate::escape;

extern "C" {
    fn getwinsizeROW() -> usize;
    fn getwinsizeCOL() -> usize;

}
pub fn get_terminal_size() -> [usize; 2] {
    let x: [usize; 2];

    unsafe {
        x = [getwinsizeCOL(), getwinsizeROW()];
    }
    return x;
}

pub fn set_terminal_name(name: &str){
    println!("\x1B]30;{}\0",name);
}
pub fn set_terminal_size(x: usize, y: usize) {

    print!("\x1B[8;{};{}t",y,x);
    
}
