fn destroy_box(c: Box<i32>){
    println!("Destroying box that contains {}",c);
}

fn main(){
    let x = 5u32;
    let y = x;

    println!("x is {}, y is {}",x,y);

    let a = Box::new(5i32);

    println!("a contiains {}",a);

    let b = a;
    destroy_box(b);
}
