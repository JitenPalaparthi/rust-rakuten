fn main() {
    
    let touple1=(100,"Hello World",123.1231); 
    println!("area of rect:{}",area_rect(12.34,13.45));
    println!("perimeter of rect:{}",perimeter_rect(12.34,13.45));

    let (a1,p1) = rect(12.34,13.45);

    println!("Area of rect:{}",a1);
    println!("Perimeter of rect:{}",p1);

    let touple2 = rect(12.34,13.45);

    println!("Area of rect:{}",touple2.0);
    println!("Perimeter of rect:{}",touple2.1);
}


fn area_rect(l:f32,b:f32)->f32{
    l*b
}


fn perimeter_rect(l:f32,b:f32)->f32{
    return 2.0*(l+b);
}

fn rect(l:f32,b:f32)->(f32,f32){
    let area = area_rect(l,b);
    let perimeter= perimeter_rect(l,b);
    (area,perimeter)
}
