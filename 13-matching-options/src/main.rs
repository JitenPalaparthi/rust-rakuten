fn main() {
    println!("Hello, world!");

    let number: Option<i32> = None;

    // enum Option<T>
    // Some(T)
    // None

    // number.expect("None found");

    // find the square of a given number
    match number {
        Some(n) => {
            println!("Square of a number is {}", n * n)
        }
        None => {
            println!("Cannot evaluate square for a None")
        }
    }

    println!("Has some {}", number.is_some());

    let n1: Option<i32> = Some(12);
    println!("{}", square(n1));
}

fn square(number: Option<i32>) -> i32 {
    match number {
        Some(n) => {
            //println!("Square of a number is {}",n*n)
            return n * n;
        }
        None => {
            // println!("Cannot evaluate square for a None")
            return -1;
        }
    }
}

fn square1(number: Option<i32>) -> Option<i32> {
    let mut sq: Option<i32> = None;
    match number {
        Some(n) => {
            //println!("Square of a number is {}",n*n)
            return Some(n * n);
        }
        None => {
            // println!("Cannot evaluate square for a None")
            return None;
        }
    }
}

// There is no Null ,null or nil in rust
// To consider whether a variable has a value or not , one can use Option type
// Two enumarated values for Option. 1. Some 2. None

