use crate::sys;

fn check_win(){
    #[cfg(windows)]
    sys::windows::enable_virtual_terminal_processing();
}

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