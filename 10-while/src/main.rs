fn main() {
    let mut a = 1;
    let mut b = 1;
    let mut count = 1;

    print!("{} {} ", a, b); // 1 1
    'outer:while true {
        if count==20{
            break 'outer;
        }
        let t = a; 
        a = b; 
        b = a + t;
        print!("{} ", b);
        count += 1;
    }
}
