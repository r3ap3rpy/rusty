use std::path::Path;

fn main(){
    let path = Path::new(".");

    let _display = path.display();

    let new_path = path.join("a").join("b");

    match new_path.to_str(){
        None => panic!("New path is not of UTF8 type."),
        Some(s) => println!("new path is {}",s),
    
    }

}
