//! rust side of the c script
//#[link(name="key", kind="static")]

extern{
    fn getkey() -> usize; 
}
pub fn call() -> usize {
    #[cfg(windows)]
    extern{
        fn getkey() -> usize; 
    }
    
    unsafe {
        if cfg!(windows){
            #[cfg(windows)]
            {
            return getkey();
            }
            //return 0;
        }
    }
    return 0;
}

pub fn call2() -> usize{
   
    #[cfg(linux)]
    extern {
        fn getkeyL() -> usize;
    }

    
    unsafe {
        if cfg!(linux){
            #[cfg(linux)]
            {
                return 0;
            }
        }else{
            return 0;
        }
    }
    return 0;
    
}

pub fn get_key() -> usize {
    //println!("ok");
    #[cfg(windows)]
    {
        return call()
    }

    #[cfg(linux)]
    {
        return call2()
    }

    //return 0;
}

