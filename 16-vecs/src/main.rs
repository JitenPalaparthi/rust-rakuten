fn main() {
    
    let mut s1: String = String::from("Hello");

    s1.push_str(" World");


    let mut vec1:Vec<i32>=vec![10,12,13,14];

    vec1.push(10);
    vec1.push(100);
    vec1.push(201);

    let slice1:&[i32]= &vec1[0..3]; //slice is heap allocated 

    let arr1:[i32;4]=[10,11,12,14];

    let slice2=&arr1[0..]; // slice is stack allocated

    for i in vec1{
        print!("{} ",i);
    }

    let mut vec2:Vec<i32> = Vec::new();

    vec2.push(10);
    vec2.push(100);
    vec2.push(201);

    for i in vec2{
        print!("vec2:{} ",i);
    }

    let mut vec3:Vec<String> = vec!["Hello".to_string(),"World".to_string(),"!".to_string()];
    let mut vec4:Vec<&str> = vec!["Hello","World","!"];

    
    


}

// vectors are not fixed size unlike arrays
// vectors are heap allocated
