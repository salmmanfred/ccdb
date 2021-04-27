// checks so everything is in order
use crate::sys;
pub fn thread_check(threads: i8, lenght: i64) {
    // thread check
    if threads as i64 > lenght {
        panic!("More Threads than lines")
    }
    if threads % 2 == 1 && threads != 1 {
        panic!("Threads are odd")
    }
}

pub fn check_win(){
    if cfg!(windows){
        #[cfg(windows)]
        sys::windows::enable_virtual_terminal_processing();
    }
}