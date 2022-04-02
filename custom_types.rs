// struct
// enum
// const, static
#[derive(Debug)]
struct Person{
    name: String,
    age: u8,
}
struct Unit;
struct Pair(i32,f32);
struct Point{
    x: f32,
    y: f32,
}
struct Rectangle{
    bottom_right: Point,
    top_left: Point,
}

enum WebEvent{
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click{x:i64,y:i64}
}

fn inspect(event: WebEvent){
    match event {
        WebEvent::PageLoad => println!("page loaded!"),
        WebEvent::PageUnload => println!("page unloaded"),
        WebEvent::KeyPress(c) => println!("pressed {}",c),
        WebEvent::Paste(s) => println!("Pasted {}",s),
        WebEvent::Click{x,y} => println!("clicked at {} and {}",x,y),
    }
}

enum Number {Zero,One,Two}
static LANGUAGE: &str = "Rust";
fn main(){
    let name = String::from("Daniel");
    let age = 31;
    let daniel = Person{name, age};
    println!("Person: {:?}",daniel);

    let point: Point = Point {x:10.3,y:11.6};
    println!("The Point coordinates, x: {}, y: {}",point.x,point.y);

    let pair = Pair(1, 0.1);
    println!("The pair looks like: {:?} : {:?}",pair.0,pair.1);

    let pressed = WebEvent::KeyPress('x');
    let click = WebEvent::Click{x:30,y:20};
    inspect(pressed);
    inspect(click);
    println!("zero is {}", Number::Zero as i32);
    println!("This is {}",LANGUAGE);
}
