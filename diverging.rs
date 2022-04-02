fn foo() -> ! {
    panic!("This call never returns!");
}

fn some_fn() {
    ()
}

/*#[feature(never_type)]
fn main(){
    let x: ! = panic!("This call never returns!");
    println!("You will never see this line!");
}*/

//fn main(){
//    let a: () = some_fn();
//    println!("This function never returns, and this line is visible!");
//}

fn main(){
    fn sum_odd_numbers(up_to: u32) -> u32 {
        let mut acc = 0;
        for i in 0..up_to {
            let addition: u32 = match i%2 == 1 {
                true => i,
                false => continue,
            };
            acc += addition;
        }
        acc
    }
    println!("The sum of odd numbers up to 9 (excluding): {}",sum_odd_numbers(9));
}
