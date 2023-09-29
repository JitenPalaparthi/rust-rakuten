fn main() {
    let day = 41;

    match day {
        1 => println!("Sunday"),
        2 => println!("Monday"),
        3 => println!("Tuesday"),
        4 => println!("Wednesday"),
        5 => println!("Thursday"),
        6 => println!("Friday"),
        7 => println!("Saturday"),
        _ => println!("no day"), // similar to default case
    }

    let number = 32;

    // match number{
    //     _ =>(),
    // }

    match number {
        n if n % 2 == 0 => {
            if n%4==0 && n%8==0{
                println!("{} is divisible by 2,4 and 8", n)
            }else{
            println!("{} is divisible by 2", n)
            }
        }
        n if n % 3 == 0 => {
            println!("{} is divisible by 3", n)
        }
        number if number % 7 == 0 => {
            println!("{} is divisible by 7", number)
        }

        _ => (),
    }

    let ch: char = 'a';

    match ch {
        'a' | 'e' | 'i' | 'o' | 'u' => println!("ch is vovel"),
        _ => (),
    }
}
// there is no switch case in rust.
// similar construst in rust is pattern matching
