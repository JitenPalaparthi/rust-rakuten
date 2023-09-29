fn main() {
    // let mut a = 100; // ------------------

    // let b = &a; // immutable reference    // ------------------

    // let c = &a; // immutable

    // let d = &a; // immutable

    // // a = 200;

    // println!("{} {} {} {}", a, b, c, d);

    let mut a = 100;
    //{
        let b: &mut i32 = &mut a;
        {
            *b = *b + 1;
            println!("b:{}", b);
        }
   // }

    let c: &mut i32 = &mut a;
    {
        *c = *c + 1;
        println!("c:{}", c);
    }
    // ---------------- since out of the scope, ownership is back to a
    a = a + 100;
    println!("a:{}", a); //------------------------------s-----

    // let mut k = 1000;
    // println!("{}", k);
    // increment(&mut k);
    // println!("{}", k);
    // k += 1;
    // println!("{}", k);

    // let mut a = 100;

    // let b = &mut a; // immutable reference

    // let c = &a; // immutable

    // let d = &a; // immutable

    //  *b= 200;

    // println!("{} {} {} {}",a,b,c,d);


//     let r;                // r starts -------------------------
//     {                                                      
//         let x = 100;  // x scope starts ------------
//         r = &x; // using the refenrece of x beyond the scope 
//     }                      // x scope ends --------------
// // &x is an invalid refernece 
//     println!("{}",r);


} // r ends --------------------------------------------------------    



fn increment(i: &mut i32) {
    *i = *i + 1;
    //return *i;
}
