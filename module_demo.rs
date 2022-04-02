mod my_mod {
    fn private_function(){
        println!("This is my_mod::private_function()");
    }
    pub fn function(){
        println!("This is my_mod::function()");
    }
    pub fn indirect_access(){
        println!("This is the my_mod::indirect_access()");
        private_function();
    }
    pub mod nested{
        pub fn function(){
            println!("This is the my_mod::nested::function()");
        }
        #[allow(dead_code)]
        fn private_function(){
            println!("This is the my_mod::nested::private_function()");
        }
        pub(in crate::my_mod) fn public_function_in_nested(){
            println!("This is my_mod::nested::public_function_in_nested()");
            public_function_in_nested();
        }
    }
}
mod my {
    pub struct OpenBox<T>{
        pub contents: T,
    }
    #[allow(dead_code)]
    pub struct ClosedBox<T>{
        contents: T,
    }
    impl<T> OpenBox<T> {
        pub fn new(contents: T) -> ClosedBox<T> {
            ClosedBox {contents: contents}
        }
    }
}

/*use create::deepply::nested::{
    my_first_function,
    my_second_function,
    AndATraitType,
};
 */
//use deeply::nested::function as other_function
fn main(){
    my_mod::function();
    my_mod::indirect_access();
    my_mod::nested::function();
    //my_first_function();
    let open_box = my::OpenBox {contents: "public information!"};
    println!("The open box content: {}",open_box.contents);
    //let _closed_box = my::ClosedBox::new("classified information!");
    //println!("This is top secret: {}",_closed_box.contents)
}
