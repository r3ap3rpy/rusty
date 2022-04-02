fn drink(beverage: &str) {
    if beverage == "beer" { panic!("You cant drink beer!"); }
    println!("You can drink {}",beverage);
}

fn main(){
    drink("water");
    drink("beer");
}
