fn main() {
    let e1:Empty=Empty{};
   // let e = 100; // this is a primitive type
    println!("{:?}",e1); // this is designed to print primitive types
    e1.print(); 
    Empty::print_direct();
}

#[derive(Debug)] // this implements Debug trait
struct Empty;
// struct, enum
// 1- empty struct
// 2- ? Why do we need empty struct?  
impl Empty {
    fn print(&self) {
        println!("there is nothing to print");
    }

    fn print_direct(){
        println!("there is nothing to print");
    }
    // self and Self
   // fn new() // the ideamatic approach of rust is using new function
}
