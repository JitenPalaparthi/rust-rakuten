use std::any::type_name;

fn main() {
    // a scope can be evaluated and convereted as an expression
   let c= {
       let a:i32= 100;
       let b:i32 =200;
       let c:i32= a+b; 
        c
    };

    println!("{}",c); // for c who has implemented Display trait 

    let emptytuple = (); // it has not implemented Display trait

    println!("{:?}",emptytuple);
    println!("{}",type_of(c));

    let k1 = what_is_my_scope(); // copy
    // k1-> 0x111144
    
}

fn type_of<T>(_:T)-> &'static str{ // back quote
   type_name::<T>()
}

fn what_is_my_scope()->i32{ 
    let k=100; // 0x110011
    k
}

// difference between expression and statement
// task: Try to return a tuple from a scope
