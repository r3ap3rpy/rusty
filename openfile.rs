use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main(){
    let path = Path::new("hello.txt");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("cant open"),
        Ok(file) => file,

    };
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("Cant read!"),
        Ok(_) => println!("File {} contains {}",display,s),
    }
}
