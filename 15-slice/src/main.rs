fn main() {
    let arr1 = [10, 11, 12, 7, 3, 34, 34, 6, 4, 78, 56, 89, 98];

    let slice1 = &arr1[0..3];

    let slice2 = &arr1[4..];

    let slice3 = &arr1[..];

    let slice4 = &arr1[0..=3]; // this inclides 4rd element as well

    println!("arr1={:?}", arr1);
    println!("slice1={:?}", slice1);
    println!("slice2={:?}", slice2);
    println!("slice3={:?}", slice3);
    println!("slice4={:?}", slice4);

    let mut str1 = "Hello World";
    str1 = "Hello World!";

    let str2 = "Hello World".to_string(); // ptr and len
                                          // let str3 = str2 // str2 is no logner the owner for the data
    {
        let str3 =&str2; 
        println!("str2 length:{}", len_of_a_string(str3)); // heap allocation
    }

    println!("str1 length:{}", len_of_a_string(str1)); // stack allocation
    println!("str2 length:{}", len_of_a_string(&str2)); // heap allocation

    println!("str1 length:{}", len_of_a_String(&str1.to_string())); // stack allocation
    //println!("str2 length:{}", len_of_a_String(str2)); // heap allocation

    // {
    // let str4 = str2;
    // }

    let mut x = 100;
    {
        let mut y = &mut x; // This is no heap allocation hence it is a copy
        *y+=100;
        println!("owner of x:{}",x);
    }
    println!("owner of x:{}",x);

    println!("str2 length:{}", len_of_a_String(&str2)); // heap allocation
    println!("str2 length:{}", len_of_a_String(&str2)); // heap allocation

}

fn sum_of(slice: &[i32]) -> i32 {
    let mut sum = 0;
    for v in slice {
        sum += *v;
    }
    sum
}

fn len_of_a_string(s: &str) -> i32 {
    return s.len() as i32;
}

fn len_of_a_String(s: &String) -> i32 {
    return s.len() as i32;
}
