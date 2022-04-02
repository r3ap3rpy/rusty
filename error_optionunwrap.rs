fn give_adult(drink: Option<&str>) {
    match drink {
        Some("lemonade") => println!("Yuck too sweet!"),
        Some(inner) => println!("{} this is nice",inner),
        None => println!("Nothing to drink?!"),
    }
}

fn drink(drink: Option<&str>) {
    let inside = drink.unwrap();
    if inside == "beer" { panic!("You cant do that!"); }
    println!("I love {}",inside);
}

fn main(){
    let water = Some("water");
    let beer = Some("beer");
    let void = None;

    give_adult(water);
    give_adult(beer);
    give_adult(void);

    let coffee = Some("coffee");
    let nothing = None;

    drink(coffee);
    drink(nothing);
}
