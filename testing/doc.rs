#![crate_name = "doc"]

pub struct Person{
    name: String,
}

impl Person {
    pub fn new(name: &str) -> Person {
        Person {
            name: name.to_string(),
        }
    }
    pub fn hello(&self) {
        println!("Hello, {}!",self.name);
    }
}

#[doc(inline)]
pub use bar::Bar;

/// Documentation for bar
mod bar {
    /// the documentation for the Bar
    pub struct Bar;
}

#[doc(no_inline)]
pub use crate::mem::drop;

#[doc(hidden)]
pub use self::async_await::*;

fn main(){
    let daniel = Person::new("Daniel");
    daniel.hello();
}
