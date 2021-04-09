
//! rust side of the c script
//#[link(name="key", kind="static")]
extern {
    fn getkeyd()->usize;
    fn keydownd()->usize;

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