// #[attribute(value1,value2,value3,etc...)]i
#![crate_type="lib"]
#![crate_name="rary"]
#[cfg(target_os="linux")]
fn are_you_on_linux(){
    println!("You are on linux!");
}
#[cfg(not(target_os="linux"))]
fn are_you_on_linux(){
    println!("You are NOT on linux!");
}
#[allow(dead_code)]
fn function(){
    println!("This will never be used!")
}

fn main(){
    println!("Dead code!");
    are_you_on_linux();
    if cfg!(target_os="linux"){
        println!("Yeah I am sure!");
    }else{
        println!("Not so sure anymore!");
    }
}

