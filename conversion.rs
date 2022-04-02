use std::fmt;
use std::convert::From;
use std::convert::TryFrom;
use std::convert::TryInto;

#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

struct Circle {
    radius: i32,
}

#[derive(Debug)]
struct FNumber{
    value: i32,
}

#[derive(Debug)]
struct INumber {
    value: i32,
}

impl TryFrom<i32> for EvenNumber {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, Self::Error>{
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        }
        else {
            Err(())
        }
    }
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"Circle of radius: {}",self.radius)
    }
}

impl From<i32> for FNumber {
    fn from(item: i32) -> Self {
        FNumber {value: item}
    }
}

impl From<i32> for INumber {
    fn from(item: i32) -> Self {
        INumber {value: item}
    }
}

fn main(){
    let fnum = FNumber::from(30);
    println!("My FNumber is {:?}",fnum);

    let i = 10;
    let inum: INumber = i.into();
    println!("My INumber is {:?}",inum);

    let circle = Circle {radius: 6};
    println!("{}",circle.to_string());

    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;
    println!("Sum: {:?}",sum);

    assert_eq!(EvenNumber::try_from(8),Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5),Err(()));

    let result: Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(result,Ok(EvenNumber(8)));
    let result: Result<EvenNumber, ()> = 5i32.try_into();
    assert_eq!(result,Err(()));
}
