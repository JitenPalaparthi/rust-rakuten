fn main() {



   let s3= largest1("Hello","World!");
   println!("{}",s3);

   let s4= largest1("Hello,","World");
   println!("{}",s4);

   let s5 = largest2("Hello", "World");
   println!("{}",s5);
}

// when ever a function is taking multiple references as arguments , need to specify explicit lifetimes
// to indicate how long those references are valid.
fn largest1<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        return s1;
    }
    return s2;
}

fn largest2<'a,'b:'a,'c:'a>(s1: &'a str, s2: &'b str) -> &'a str { // lifetime binding
    let s3:&'c str="Hey Rakutan";
    if s1.len() > s2.len() {
        return s1;
    }
    return s3;
}

fn largest4<'a>(s1: &'a str, s2: &'a str) -> &'a str { // lifetime binding
    let s3:&'a str="Hey Rakutan";
    if s1.len() > s2.len() {
        return s1;
    }
    return s3;
}



fn largest3(s1: &'static str, s2: &'static str) -> &'static str {

    if s1.len() > s2.len() {
        return s1;
    }
    return s2;
}


