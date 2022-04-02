fn multiply(first_number: &str, second_number: &str) -> i32 {
    let first_number = first_number.parse::<i32>().unwrap();
    let second_number = second_number.parse::<i32>().unwrap();
    first_number * second_number
}

fn main(){
    let twenty = multiply("10","2");
    println!("double is {}",twenty);
    let tt = multiply("t","2");
    println!("double is {}", tt);
}
