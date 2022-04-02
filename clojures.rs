// |val| val + x
//
fn main(){
    fn function(i: i32) -> i32 { i + 1}

    let clojure_annotated = |i: i32| -> i32 {i+1};
    let clojure_inferred = | i | i+1 ;
    let i = 1;
    println!("function: {}",function(i));
    println!("annotated: {}",clojure_annotated(i));
    println!("inferred: {}",clojure_inferred(i));
}
