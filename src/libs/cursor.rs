//! rust side of the c script

use crate::escape;
use crate::sys;




pub fn gotoxy(x: i64, y: i64) {
    check_win();


    escape::set_cursor_pos(x as i64,y as i64);
}
pub fn clear() {
    check_win();
    gotoxy(0,0);
    

}
pub fn mega_clear(){
    check_win();
    escape::clear();
}
pub fn hide_cursor() {
    check_win();


    escape::hide_cursor();
}
pub fn show_cursor(){
   
    check_win();
    escape::show_cursor();
}

fn check_win(){
    #[cfg(windows)]
    sys::windows::enable_virtual_terminal_processing();
}
