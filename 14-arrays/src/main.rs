fn main() {
    let num1: i32 = 10;

    let num2: Option<i32> = Some(10);

    let mut arr1: [i32; 3] = [10, 11, 12]; // stack allocated
                                           // Is the avove array  mutable or not?
    arr1[0] = 100;
    println!("{:?}", arr1);

    // assign default values to the array
    let arr2: [i32; 100] = [10; 100];

    println!("{:?}", arr2);

    println!("Size of arr2:{}", std::mem::size_of_val(&arr2));

    println!("Length of arr2:{}", arr2.len());

    let arr3=[0,12,14,3,45]; // the length is evaluated at the compile time based on the values assigned to the array


    // for loop

    let mut i = 0;
    for v in arr1 {
        println!("Index:{} Value:{}", i, v);
        i += 1;
    }

    println!("Sum of arr1:{}",sum_of(arr1));
    //println!("Sum of arr3:{}",sum_of(arr3));
   // println!("Sum of arr1:{}",sum_of(arr2));

    // &str

   


}

fn sum_of(a: [i32; 3]) -> i32 {
    let mut sum = 0;
    for v in a {
        sum += v;
    }
    sum
}

// fn sum_of(a: [i32; 3]) -> i32 {
//     let mut sum = 0;
//     for v in a {
//         sum += v;
//     }
//     sum
// }


// arrays are allocated in stack memory in rust
// arrays are fixed size
// no automatic / default type inference for any types in Rust but
// you can infer a default value to all elements of an array
