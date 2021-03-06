fn reverse(pair: (i32,bool)) -> (bool, i32) {
    let (integer, boolean) = pair;
    (boolean,integer)
}

#[derive(Debug)]
struct Matrix(f32,f32,f32,f32);

fn main(){

    let long_tuple = (1u8,2u16,3u32,4u64,-1i8,-2i16,-3i32,-4i64,0.1f32,0.2f64,'a',true);

    println!("First value of the tuple: {}",long_tuple.0);
    println!("The second value of the tuple: {}",long_tuple.1);

    let pair = (1,true);
    println!("pair is {:?}",pair);
    println!("The reversed pair is: {:?}",reverse(pair));

    println!("One element tuple: {:?}",(5u32,));
    let tuple = (1,"hello",4.5,true);

    let (a,b,c,d) = tuple;
    println!("{:?},{:?},{:?}",a,b,c);
    let matrix = Matrix(1.1,1.2,1.3,1.4);
    println!("{:?}",matrix);
}
