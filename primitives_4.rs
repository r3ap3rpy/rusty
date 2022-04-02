use std::mem;

fn analyze_slice(slice: &[i32]){
    println!("first element {}",slice[0]);
    println!("the length {}",slice.len());
}

fn main(){
    let xs: [i32;5] = [1,2,3,4,5];
    let ys: [i32;500] = [0;500];

    println!("The first element: {}",xs[0]);
    println!("The length is {}",xs.len());

    analyze_slice(&xs);

    analyze_slice(&ys[1..4]);
}
