use std::fmt;

fn main() {
    println!("Hello, world!");
    let r1: Rect = Rect::new(10.23, 12.45);
    println!("Area of r1:{}\n Perimeter of r1:{}",r1.area(),r1.perimeter());
    println!("{}", r1);
  //  r1.drop();
   // drop(r1);
    {
        let r2: Rect = Rect::new(24.45, 23.54);
        println!("{}", r2);
        println!("Area of r2:{}\n Perimeter of r2:{}",r2.area(),r2.perimeter());
    }
}

struct Rect {
    l: f32,
    b: f32,
}

impl Rect {
    fn new(l: f32, b: f32) -> Self {
        Rect { l: l, b: b }
    }
    fn area(&self)->f32{
        self.l*self.b
    }
    fn perimeter(&self)->f32{
        2.0*(self.l+self.b)
    }
}

impl fmt::Display for Rect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Length:{} Bredth:{}", self.l, self.b)
    }
}

// Drop trait
impl Drop for Rect {
    fn drop(&mut self) {
        println!("rect object is dropping from the memory");
    }
}

struct Empty;

impl Drop for Empty{
    fn drop(&mut self) {
        println!("empty struct object is dropped");
    } 
}

struct Myi32(i32);

impl Drop for Myi32{
    fn drop(&mut self) {
        println!("Myi32 struct object is dropped");
    } 
}

type integer = i32; // typedef in c or c++


// result::Result<(), Error>
