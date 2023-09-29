fn main() {
    let s1 = Shape::Square(25.25);
    let c1 = Shape::Circle(12.5);
    let r1 = Shape::Rectangle(12.4, 13.5);
    println!("Area of Square:{}",area(&s1));
    println!("Area of Circle:{}",area(&c1));
    println!("Area of Rectangle:{}",area(&r1));
}

#[derive(Debug)]
enum Shape {
    Square(f32),
    Rectangle(f32, f32),
    Circle(f32),
}

fn area(shape: &Shape) -> f32 {
    match shape {
        Shape::Circle(r) => {
            let a = 3.14 * r * r;
            return a;
        }

        Shape::Square(s) => s * s,
        Shape::Rectangle(l, b) => l * b,
    }
}
