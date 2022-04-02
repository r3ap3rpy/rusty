// sclar types
// signed integer()i
// signed integer(u8,u16,u32,u64,u128,usize)
// float (f32,f64)
// char 'a','b','c'
// bool (true, false)
// ()
// compound types
//  arrays [1,2,3]
//  tuples (1, true)
//
fn main(){
    let logical: bool = true;
    let a_float: f64 = 1.0;
    let default_int = 7;

    let mut mutable = 12;

    println!("{} mut value",mutable);
    mutable = 24;
    println!("{} mut value",mutable);
}
