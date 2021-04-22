
//! rust side of the c script
//#[link(name="key", kind="static")]
extern {
    fn getkeyd()->usize;
    fn keydownd()->usize;
    fn safekey()->usize;

}

pub fn call() -> usize{
    unsafe {
        return getkeyd();
    }
}

pub fn keydown() -> bool{
    unsafe{
        if keydownd() == 1{
            return true
        }
        else {
            return false
        }
    }
}

pub fn getkey()->usize{
    //println!("ok");
    call()
}
pub fn safegetkey()->usize{// not really safe but eh
    let mut a = 0;
    unsafe {
        a = safekey()
    }
    return a
}