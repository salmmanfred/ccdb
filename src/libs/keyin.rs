//! rust side of the c script
//#[link(name="key", kind="static")]
extern "C" {
    fn getkeyd() -> usize;
    fn keydownd() -> usize;
    fn safekey() -> usize;

}

pub fn call() -> usize {
    unsafe {
        return getkeyd();
    }
}

pub fn key_down() -> bool {
    unsafe {
        if keydownd() == 1 {
            return true;
        } else {
            return false;
        }
    }
}

pub fn get_key() -> usize {
    //println!("ok");
    call()
}
pub fn safe_get_key() -> usize {
    // not really safe but eh
    let mut a = 0;
    unsafe { a = safekey() }
    return a;
}
