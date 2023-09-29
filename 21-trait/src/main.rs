fn main() {
    println!("Hello, world!");
    let c1:Cuboid=Cuboid { l: 10.24, b: 12.,h:12.34 };
    let r1:Rect=Rect { l: 10.24, b: 12.45 };
    let s1:Square= Square(12.25);

    area(&c1);
    area(&r1);
    area(&s1);

}

fn area(a:&impl Area){ // impl dyn
    println!("Area:{:.3}",a.area())
}

trait Area {
    fn area(&self) -> f32;
}

struct Rect {
    l: f32,
    b: f32,
}

impl Area for Rect {
    fn area(&self) -> f32 {
        self.l * self.b
    }
}

struct Cuboid{
    l:f32,
    b:f32,
    h:f32,
}

impl Area for Cuboid{
    fn area(&self) -> f32 {
        self.l * self.b * self.h
    }
}

struct Square(f32);

impl Area for Square {
    fn area(&self) -> f32 {
        self.0 * self.0
    }
}
