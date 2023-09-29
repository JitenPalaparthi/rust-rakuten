pub fn greet(){
    println!("Hello World from greet func from mymodule");
}

#[derive(Debug)]
pub struct G; //unit struct 

pub mod my_module{
    pub fn greet(){
        println!("Hello World from greet func from mymodule/my_module");
    } 
}