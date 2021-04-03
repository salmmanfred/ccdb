
//! rust side of the c script
//#[link(name="key", kind="static")]
extern {
    fn getkeyd()->usize;
}
pub fn call() -> usize{
    unsafe {
        return getkeyd();
    }
}

pub fn getkey()->usize{
    //println!("ok");
    call()
}