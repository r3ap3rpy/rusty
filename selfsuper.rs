fn function(){
    println!("This is an outside funciton!");
}
mod cool {
    pub fn function(){
        println!("The function of cool module cool::function()!");
    }
}
mod my {
    fn function(){
        println!("The function of my module my::function() which is private!");
    }
    mod cool {
        pub fn function(){
            println!("This is the function of my::cool::function()");
        }
    }
    pub fn indirect_call(){
        println!("This is an indirect_call function my::indirect_call()");
        self::function();
        function();
        self::cool::function();
        super::function();
        {
            use crate::cool::function as root_function;
            root_function();
        }
    }
}
fn main(){
    my::indirect_call();
}
