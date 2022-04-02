pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[allow(dead_code)]
fn bad_add(a: i32, b: i32) -> i32 {
    a - b
}

fn sqrt(number: f64) -> Result<f64, String> {
    if number >= 0.0 {
        Ok(number.powf(0.5))
    } else {
        Err("Negative floats do not have a square root!".to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add(){
        assert_eq!(add(1,2),3);
    }
    #[test]
    fn test_bad_add(){
        assert_eq!(bad_add(1,2),3);
    }
    #[test]
    fn test_sqrt() -> Result<(), String>{
        let x = 4f64;
        assert_eq!(sqrt(x)?.powf(2.0),x);
            Ok(())
    }
    #[test]
    #[ignore]
    fn ignored_test(){
        assert_eq!(add(1,2),4);
    }
}
