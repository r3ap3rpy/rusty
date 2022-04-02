fn double_first(vec: Vec<&str>) -> i32{
    let first = vec.first().unwrap();
    2 * first.parse::<i32>().unwrap()
}

fn main(){
    let numbers = vec!["42","93","18"];
    let empty = vec![];
    let strings = vec!["foo","bar","18"];

    println!("{}",double_first(numbers));
    println!("{}",double_first(empty));
    println!("{}",double_first(strings));
}
