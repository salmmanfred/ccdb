use std::process::Command;


extern {
    fn getwinsize() -> [usize; 2];
    fn setwinsize();

}
pub fn getTerminalSize()->[i64;2]{
    let x: [i64;2];
    unsafe{
        x = [getwinsize()[0] as i64, getwinsize()[1] as i64];
    }
    return x;
}
pub fn setTerminalSize(x:usize,y:usize){
    //mode con:cols=80 lines=100
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
                .args(&["/C",&format!("mode con:cols={} lines={}",x,y)])
                .output()
                .expect("failed to execute process")
    } else {
        panic!("This function is not yet supported on linux!");
    };
}