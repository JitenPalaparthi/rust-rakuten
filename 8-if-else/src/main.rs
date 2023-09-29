fn main() {
    let num = 16;
    // == && || != !
    if num % 2 == 0 {
        println!("even number")
    } else {
        println!("odd number")
    }

    // if num%2==0 && num%4==0 && num%8==0{
    if num % 8 == 0 {
        println!("{} is divisible by 2,4 and 8", num);
    } else if num % 4 == 0 {
        println!("{} is divisible by 2,4 ", num);
    } else if num % 2 == 0 {
        println!("{} is divisible by 2 ", num);
    } else {
        println!("{} is not divisible by 2,4 and 8 ", num);
    }

    // a simple task to know if and else
    // state,    age, Height ,gender , ticket
    // AP        5  or >3Feet    M       To take ticket
    // AP        10  or >4feet   F       To take ticket
    // Tamilnadu 6  or >3.5Feet  M     To take ticket
    // Tamilnadu -   -           F      no ticket

   let result =  if num % 8 == 0 { // returns a value to a variable becase if block acts as an expression
         format!("{} is divisible by 2,4 and 8", num)
    } else if num % 4 == 0 {
        format!("{} is divisible by 2,4 ", num)
    } else if num % 2 == 0 {
        format!("{} is divisible by 2 ", num)
    } else {
        format!("{} is not divisible by 2,4 and 8 ", num)
    }; // This tells that the whole block is an expression

    println!("{}",result);

}

// 1. Stack Heap allocations
// 2. Ownership and moves
// 3- Borrowing
// 4- Lifetimes
// 5- Traits
// 6- Strings
// 7- There is no Null or nil in rust
