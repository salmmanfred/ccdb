use openfile;
pub struct map{
    pub chars: Vec<String>,
    pub x: Vec<i64>,
    pub y: Vec<i64>,
}
// this parses a text file into a map 

pub fn load(filename: &str) -> map{
    let file = openfile::readFileLines(filename);
    loadvec(file)
}
pub fn loadvec(file: Vec<String>) -> map{
    let mut x: Vec<i64> = Vec::new();
    let mut y: Vec<i64> = Vec::new();
    let mut c: Vec<String> = Vec::new();
    let mut yy = 0;
    let mut xx = 0;

    for i in file{
        
        for ii in i.chars(){
            // ignore the spaces 
            if ii != ' '{
                x.push(xx);
                y.push(yy);
                c.push(ii.to_string())
            }
            xx+=1;
        }
        xx = 0;
        yy+=1;
    }

    map{
        chars: c,
        x:x,
        y:y,
    }
}
pub fn toMap(str: String)-> map{
    let sttr = str;
    let mut vecStr: Vec<String> = Vec::new();
    let vecsttr: Vec<&str> = sttr.split("\n").collect();
    for x in vecsttr{
        vecStr.push(x.to_string());

    }

    loadvec(vecStr)
}