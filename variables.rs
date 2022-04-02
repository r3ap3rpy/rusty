fn main(){
    let an_integer = 11;
    let a_boolean = true;
    let unit = ();

    let copied = an_integer;
    let _will_complain = true;
    println!("The integer {}",an_integer);
    println!("The boolean {}",a_boolean);
    println!("The unit value {:?}",unit);
    println!("Copied {}",copied);

    let mut increase = 1;
    println!("The value of increase: {}",increase);
    increase += 1;
    println!("The increased value {}",increase);
    let global_variable = 1;
    println!("Value of global: {}",global_variable);
    {
        let short_life = 10;
        let global_variable = 11;
        println!("The life is short: {}",short_life);
        println!("Value of global: {}",global_variable);
    }
    let later;
    {
        let x = 2;
        later = x * x;
        println!("The value later {}",later);
    }
    let mut _mutable_integer = 31;
    {
        let _mutable_integer = _mutable_integer;
        // below errors out
        //_mutable_integer = 50;
    }
    _mutable_integer = 3;
    println!("Now it works: {}",_mutable_integer);
}
