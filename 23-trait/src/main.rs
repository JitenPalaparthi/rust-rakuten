fn main() {
    println!("Hello, world!");

    let mut t1: Box<dyn Area> = Box::new(Square::new(10.25));

    let r1: Rect = Rect::new(10.24, 13.45);

    println!("Area of Square:{:.2}", t1.area());
    t1.print();
    t1.who();


    t1 = Box::new(r1);
    println!("Area of Rect:{:.2}", t1.area());
    t1.print();
    t1.who();
    // why should we use box and why to use dyn
}

trait Area {
    fn area(&self) -> f32;
    fn print(&self);
    fn who(&self) {
        println!("I am area function");
    }
}

struct Square(f32);

struct Rect {
    l: f32,
    b: f32,
}

impl Square {
    fn new(s: f32) -> Self {
        //Self(s)
        return Square(s);
    }
}

impl Area for Square {
    fn area(&self) -> f32 {
        self.0 * self.0
    }
    fn print(&self) {
        println!("Square Side:{:.2}", self.0)
    }
}

impl Rect {
    fn new(l1: f32, b1: f32) -> Self {
        Self { l: l1, b: b1 }
    }
}

impl Area for Rect {
    fn area(&self) -> f32 {
        self.l * self.b
    }
    fn print(&self) {
        println!("Rect Length:{:.2} Rect Bredth:{:.2}", self.l, self.b)
    }

    fn who(&self) {
        println!("I am rect object");
    }
}
