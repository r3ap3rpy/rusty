use std::mem;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

#[allow(dead_code)]
struct Rectangle{
    top_left: Point,
    bottom_right: Point,
}
fn origin() -> Point {
    Point { x: 0.0, y: 0.0 }
}
fn boxed_origin() -> Box<Point> {
    Box::new(Point {x: 0.0, y: 0.0})
}
fn main(){
    let point: Point = origin();
    let rectangle: Rectangle = Rectangle {
        top_left: origin(),
        bottom_right: Point {x: 3.0, y: -4.0 },
    };
    let boxed_point: Box<Point> = Box::new(origin());
    println!("Point occupies {} bytes on stack!",mem::size_of_val(&point));
    println!("Rectangle occupies {} bytes on stack!",mem::size_of_val(&rectangle));
    let boxed_rectangle: Box<Rectangle> = Box::new(Rectangle {
        top_left: origin(),
        bottom_right: Point { x: 3.0, y: -4.0 },
    });
    let box_in_a_box: Box<Box<Point>> = Box::new(boxed_origin());
    println!("Boxed point occupies {} bytes on stack!",mem::size_of_val(&boxed_point));
    println!("Boxed rectangle occupies {} bytes on stack!",mem::size_of_val(&boxed_rectangle));
    println!("Boxed box occupies {} bytes on stack!",mem::size_of_val(&box_in_a_box));
}
