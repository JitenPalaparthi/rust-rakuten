fn main() {
    println!("Hello, world!");

    let a = 100;
    let b = 200;

    if a == b {
        println!("a and b are equal");
    }

    let a1: A = A(100.1);
    let a2: A = A(100.0);

    if a1 == a2 {
        println!("a1 and b1 are equal");
    } else {
        println!("a1 and b1 are not squal")
    }

    let a3 = a1.clone();

    println!("{:?}",a3);
}

#[derive(PartialEq,Clone,Debug)]
struct A(f32);

// impl Clone for A{

// }

// impl PartialEq for A {
//     fn eq(&self, other: &Self) -> bool {
//         self.0 as i32 == other.0 as i32
//     }
// }
