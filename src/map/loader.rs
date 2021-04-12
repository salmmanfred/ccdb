use openfile;
use std::{fs, io};
use crate::sprite;

#[derive(Clone)] 
pub struct map{
    pub chars: Vec<String>,
    pub x: Vec<i64>,
    pub y: Vec<i64>,
}
impl map{
    pub fn makeSprite(&self)->sprite::sprite{
        sprite::sprite{
            chars: self.chars.clone(),
            x: self.x.clone(),
            y: self.y.clone(),

        }
    }
}
// this parses a text file into a map 

pub fn load(filename: &str) -> map{
    let file = openfile::readFileLines(filename); // calls and returns what loadvec does
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
pub fn toMap(str: String)-> map{ //makes a string into a map 
    let sttr = str;
    let mut vecStr: Vec<String> = Vec::new();
    let vecsttr: Vec<&str> = sttr.split("\n").collect();
    for x in vecsttr{
        vecStr.push(x.to_string());

    }

    loadvec(vecStr)
}

pub struct folder{
    maps: Vec<map>,
    names: Vec<String>,
    meta: Vec<i8>,
}
impl folder{
    pub fn loadAssetMap(&self, name:&str)->map{// find and push the map
        for x in 0..self.names.len(){
            if self.names[x] == name{
                if self.meta[x] == 0{
                   
                return self.maps[x].clone();
                }
            }
        }
        map{
            chars:Vec::new(),
            x:Vec::new(),
            y:Vec::new(),


        }
    }
    pub fn loadAssetSprite(&self,name:&str)->sprite::sprite{// find and push the sprite
        for x in 0..self.names.len(){
            if self.names[x] == name{
                if self.meta[x] == 1{
                   
                return self.maps[x].makeSprite();
                }
            }
        }
        sprite::sprite{
            chars:Vec::new(),
            x:Vec::new(),
            y:Vec::new(),


        }
    }
}
pub fn loadFromFolder(Directory: String, PrefixMap:String, PrefixSprite: String)->folder{
    let mut folderpr = folder{
        maps: Vec::new(),
        names: Vec::new(),
        meta: Vec::new(),
    };
    for entry in fs::read_dir(Directory).expect("Error reading folder") { // parses the folder into a folder struct 
        let entry = entry.expect("error").path().into_os_string().into_string().expect("error");
        let entry_n = entry.split("/").collect::<Vec<&str>>();//temp parse var 
        let entry_n = entry_n[entry_n.len()-1].split(".").collect::<Vec<&str>>();// temp parse var

        let entry_name = entry_n[0];//final name 

        if entry.contains(&PrefixMap){//finds all map elements 
            folderpr.maps.push(load(&entry.clone()));
            folderpr.names.push(entry_name.to_string());
            folderpr.meta.push(0);

        }
        if entry.contains(&PrefixSprite){// finding all sprite elements 
            folderpr.maps.push(load(&entry));
            folderpr.names.push(entry_name.to_string());
            folderpr.meta.push(1);

        }
       
    }
    folderpr
}