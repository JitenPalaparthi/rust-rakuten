use my_crate;
use random_number::random;
fn main() {
    let sum = my_crate::add(10, 20);
    println!("add of {}", sum);
    let n: u8 = random!();
    println!("{}", n); // 0 ~ 255
}
