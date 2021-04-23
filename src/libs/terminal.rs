use std::process::Command;


extern {
    fn getwinsizeROW() -> usize;
    fn getwinsizeCOL() -> usize;


}
pub fn get_terminal_size()->[usize; 2]{
    let x: [usize;2];
   
    unsafe{
        
        x = [getwinsizeCOL(), getwinsizeROW()];
    }
    return x;
}
pub fn set_terminal_size(x:usize,y:usize){
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