fn main(){
    //statement
    //statement
    //statement
    //
    let x = 2;
    
    let y = {
        let x_sqared = x*x;
        let x_cube = x*x*x;
        x_cube + x_sqared
    };
    let z = {
        2 * x;
    };

    println!("x is {:?}",x);
    println!("y is {:?}",y);
    println!("z is {:?}",z);
}
