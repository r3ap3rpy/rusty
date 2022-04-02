type NanoSecond = u64;


#[allow(non_camel_case_types)]
type inch = u64;


fn main(){
    let nanoseconds: NanoSecond = 5 as u64;

    println!("The nanoseconds {}",nanoseconds);

    let elem = 5u8;
    let mut vec = Vec::new();
    vec.push(elem);
    println!("{:?}",vec);
    
    let f = 1.0;
    println!("size of f in bytes is {}",std::mem::size_of_val(&f));

    let decimal = 64.4321_f32;
    //let integer: u8 = decimal;
    let integer = decimal as u8;
    println!("The values are {} {}",decimal, integer);
    println!("300.0 as u8 is {}", 300.0_f32 as u8);
    unsafe {
        println!("300.0 is {}",300.0_f32.to_int_unchecked::<u8>());
        println!("nan as u8 is {}",f32::NAN.to_int_unchecked::<u8>());
    }
    
}
