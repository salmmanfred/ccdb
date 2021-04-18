extern {
    fn getwinsize() -> [usize; 2];
}
pub fn getTerminalSize()->[i64;2]{
    let x: [i64;2];
    unsafe{
        x = [getwinsize()[0] as i64, getwinsize()[1] as i64];
    }
    return x;
}