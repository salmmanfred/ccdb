// checks so everything is in order 

pub fn threadCheck(threads: i8, lenght: i64){
    if threads as i64 > lenght{
        panic!("More Threads than lines")
    }
    if threads % 2 == 1{
        panic!("Threads are odd")
    }
}