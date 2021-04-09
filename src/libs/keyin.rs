
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

pub fn keydown() -> usize{
    unsafe{
        return keydownd()
    }
}

pub fn getkey()->usize{
    //println!("ok");
    call()
}