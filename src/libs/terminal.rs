use crate::check::check_win;
use crate::escape;

pub fn set_terminal_size(x: usize, y: usize) {
    check_win();
    print!("\x1B[8;{};{}t", y, x);
}

pub fn restore() {
    escape::restore();
}
pub fn save() {
    escape::restore();
}
