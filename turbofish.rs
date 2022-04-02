// ::<SomeType>
//

fn main(){
    let numbers: Vec<i32> = vec![10,11,12,13,14,15,16,17,18];
    let even_numbers = numbers.into_iter().filter(|n| n % 2 == 0).collect::<Vec<i32>>();

    println!("The even numbers: {:?}",even_numbers);

    let s = "Welcome to Rust!";
    let string: String = s.into();

    let sstring = Into::<String>::into(s);

    println!("{},{}",string,sstring);
}
