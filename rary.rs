pub fn public_function(){
    println!("public_function() from rary.rs");
}
fn private_function(){
    println!("private_function() from rary.rs");
}
pub fn indirect_access(){
    println!("indirect_access() from rary.rs");
    private_function();
}
