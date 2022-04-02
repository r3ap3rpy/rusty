struct Fibonacci {
    curr: u32,
    next: u32,
}

impl Iterator for Fibonacci {   
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        let new_next = self.curr + self.next;
        self.curr = self.next;
        self.next = new_next;
        Some(self.curr)
    }
}

fn fibonacci() -> Fibonacci {
    Fibonacci {curr: 0, next: 1 }
}

fn main(){
    let mut sequence = 0..3;
    println!("> {:?}",sequence.next());
    println!("> {:?}",sequence.next());
    println!("> {:?}",sequence.next());
    println!("> {:?}",sequence.next());

    for i in 0..3 {
        println!("> {}",i);
    }
}
