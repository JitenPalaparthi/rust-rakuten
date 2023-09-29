fn main() {
    let num1= Box::new(100);
    let num2 = &num1; // & reference operator. Instead of move it borrows
    

    println!("{}   {}",num1,num2);
    let a = 100;
    let b= &a;
    //
    //
    //
    let c= &a;
    println!("a:{} b:{} c:{}",a,b,c);
//  let k=110;
//  let q=k;

    let mut e= 100;
    let  d = &mut e;
    //println!("{}",e);
        *d= 100+1;
        println!("{}",e);
    let  f = &mut e;
   // *d = 100+5;
    *f=100+5;
    println!("{}",e);
    let mut s1:String = String::from("Hello World!.");
    // let s2 = s1;
    // // do something with s2
    // s1 = s2;
    s1=modify_string1(s1); // the ownership is transffered
    println!("{}",s1);

    modify_string2(&mut s1); // the ownership is transffered
    println!("{}",s1)

}

// immutable borrow
// mutable borrow


fn modify_string(mut s :String){
    s.push_str("  Some thing is written here");
    //println!("{}",s);
}
fn modify_string1(mut s :String)->String{
    s.push_str("  Some thing is written here");
    //println!("{}",s);
    s
}

fn modify_string2(s :&mut String){
    s.push_str("  Some thing is written here");
    //println!("{}",s);
}
