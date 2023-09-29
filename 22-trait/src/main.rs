fn main() {
   
   let r1:Rect = Rect{l:12.23,b:13.45};
   let s1:Square=Square(12.45);
   area_perimeter(&r1);
   area_perimeter(&s1);
}

fn area_perimeter<T: Area + Perimeter>(ap: &T) {
    println!("Area:{:.2}", ap.area());
    println!("Perimeter:{:.2}", ap.perimeter());
}

trait Area {
    fn area(&self) -> f32;
}

trait Perimeter {
    fn perimeter(&self) -> f32;
}

struct Rect {
    l: f32,
    b: f32,
}

struct Cuboid {
    l: f32,
    b: f32,
    h: f32,
}


struct Square(f32);

impl Area for Rect {
    fn area(&self) -> f32 {
        self.l * self.b
    }
}

impl Perimeter for Rect {
    fn perimeter(&self) -> f32 {
        2.0 * (self.l * self.b)
    }
}

impl Area for Square {
    fn area(&self) -> f32 {
        self.0 * self.0
    }
}

impl Perimeter for Square {
    fn perimeter(&self) -> f32 {
        self.0 * 4.0
    }
}
