//! rust side of the c script

extern {
    fn moves(x:usize,y:usize);
    fn clears();
    fn hide();
}


pub fn gotoxy(x:usize,y:usize){
    unsafe {
        moves(x,y);
    }
}
pub fn clear(){
    unsafe{
        clears();
    }
}
pub fn hideCursor(){
    unsafe{
        hide();
    }
}