// format!
// print!
// println!
// eprint!
// eprintln!
fn main(){
    println!("{} days",10);
    println!("{0} first, {1} second, {0} first",34,45);
    println!("Name arguments: {first}, {second}, {third}",first="bacon",second="cheese", third="bums");
    println!("{} of {:b} people know binary, the other half doesn't!",1,2);
    println!("Text alginment: {number:>width$}",number=10,width=5);
    println!("{number:0>width$}",number=3,width=5);
}
