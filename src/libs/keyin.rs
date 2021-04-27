//! rust side of the c script
//#[link(name="key", kind="static")]

extern "C" {
    fn getkey() -> usize;
}
pub fn call() -> usize {
    #[cfg(windows)]
    extern "C" {
        fn getkey() -> usize;
    }

    unsafe {
        if cfg!(windows) {
            #[cfg(windows)]
            {
                return getkey();
            }
            //return 0;
        }
    }
    return 0;
}

pub fn call2() -> usize {
    #[cfg(linux)]
    extern "C" {
        fn getkeyL() -> usize;
    }

    unsafe {
        if cfg!(linux) {
            #[cfg(linux)]
            {
                return getkeyL();
            }
        } else {
            return 0;
        }
    }
    return 0;
}

pub fn get_key() -> usize {
    //println!("ok");
    #[cfg(windows)]
    {
        return call();
    }

    #[cfg(target_os = "linux")]
    {
        return call2();
    }

    //return 0;
}
