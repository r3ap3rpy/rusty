fn main(){
    let n = 10;

    if n < 0 {
        println!("It's negative!");
    } else if n == 10 {
        println!("It's magic!");
    } else { 
        println!("good!");
    }

    let big_n = {
        if n < 10 && n > -10 {
            println!("We are going to double the value!");
            2 * n
        } else {
            println!("We are going to triple the value!");
            3 * n
        }
    };
    println!("{} {}",n,big_n);
    let mut count = 0i32;
    loop {
        count += 1;

        if count == 3 {
            println!("three");
            continue;
        }
        println!("{}",count);
        if count == 5 {
            println!("It's the end of the world!");
            break;
        }
    }
    'outer: loop {
        println!("Entered outer loop!");
        'inner: loop {
            println!("Entered inner loop!");
            
            break 'outer;
        }
        //println!("This will never execute!")
    }
    //println!("The outer loop is now complete!");
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The value of counter {}", counter);

    let mut numba = 1;
    while numba < 11 {
        println!("value {}",numba);
        numba+=1;

    }

    for n in 1..=10{
        println!("value {}",n);
    }

    let value = 99;
    match value {
        99 => println!("Cool!"),
        _ => println!("Nothing else matches!"),
    }

    let letter: Option<i32> = None;

    if let Some(i) = letter {
        println!("Matched {:?}!",i)
    } else {
        println!("It did not match a number go letters go!");
    }

    let mut optional = Some(0);

    while let Some(i) = optional {
        if i > 9 {
            println!("Greater than 9!");
            optional = None;
        } else {
            println!("Smaller than 9!");
            optional = Some(i+1);
        }

    }
}
