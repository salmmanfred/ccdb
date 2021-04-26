use crate::sys;
use crate::check::check_win;
//https://gist.github.com/fnky/458719343aabd01cfb17a3a4f7296797
pub fn hide_cursor(){
    check_win();
    print!("\x1B[?25l");
}
pub fn show_cursor(){
    check_win();
    print!("\x1B[?25h");
}
pub fn set_cursor_pos(x:i64,y:i64){
    check_win();
    print!("\x1B[{};{}H", x, y)
}
pub fn clear(){
    check_win();
    print!("\u{001b}c");
}
pub fn restore(){
    print!("\x1B[?47l");
}
pub fn save(){
    print!("\x1B[?47h");
}