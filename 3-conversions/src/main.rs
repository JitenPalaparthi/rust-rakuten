

fn main() {

    const PI:f32= 3.14;

    let mut num1:i32=0;
    let ok:bool=false;

    let num2 = 100u8;

    let num3 = 100; // os type i32

    let num4= 24__45__67__32__i64; // 24456732 


    num1 = 100;
    println!("num1:{} num2:{} num3:{} num4:{}",num1,num2,num3,num4);
    println!("ok:{}",ok);


    let mut num5:i32 = 12312312;

    let num6:i64 = num5 as i64;
    println!("num6:{}",num6);

    let mybox:Box<i32>=Box::new(1002);
    println!("mybox:{}",mybox);

    let num7:i64 = *mybox as i64;

    println!("num7:{}",num7);

    let num8:i64 = num2 as i32 as i64;

    println!("num8:{}",num8);
    // long num1 = (long)num2; // not this way

    let num9:i64=3123123123123123;
    // 00000000000010110001100001110110110001000101011111110011  10110011
    // 179
    let num10:u8= num9 as u8;
    println!("num10:{}",num10);

    let num11:u8=2;
    const PI2:f32 = PI*PI+2.0;
    println!("PI2:{}", PI2);

    println!("Min:{} Max:{}",u8::MIN,u8::MAX); // can get for all numaric types

    let touple1=(100,"Hello World",123.2323); 
    println!("First Value from the touple:{}",touple1.0);
    println!("Seoond Value from the touple:{}",touple1.1);
    println!("Third Value from the touple:{}",touple1.2);

}

// there is not default type inference in rust
// if a variale is declared but not initialized then it gives random values, but in rust it is must
//  let num4= 24__45__67__32__i64 : This notation is used for readability
// by default variables are immutable in rust.
// in order to reassian a value , need to use mut keyword

// type casting in rust

