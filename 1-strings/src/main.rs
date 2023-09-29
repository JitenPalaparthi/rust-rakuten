fn main() {
    let hello= "Hello உலகம்"; // string slice 

    println!("Length of the string:{}",hello.len());

    println!("bytes of hello:{:?}",hello.as_bytes()); // display bytes in a string

   // hello="Hello World";

    println!("Length of the string:{}",hello.len());

    let string1:String=String::from("Hello உலகம்"); 

    let string2:String= "Hello உலகம்".to_string();

    let string3:String = hello.to_string();

    let string4:String = string3;

   let last_char = hello.chars().nth(7).unwrap();
   println!("{}",last_char);
    //let string5:String="Hello World";

   // println!("{}",string3);

   // str str
   // String str
   // str String
   // String String
   // format!
   // concat
   // 

   let mut string1:String = "Hello".to_owned();
   let str1:&str= "World";
   string1.push_str(str1);
   println!("{}",string1);


   let mut string2:String = "Hello".to_owned();
   let str2:&str= "World";
   let string3 = string2+str2;
   println!("{}",string3);

   let mut string4:String = "Hello".to_owned();
   let str4:&str= "World";
   let string5 = format!("{} {}",string4,str4);
   println!("{}",string5);

   let mut str6:&str="Hello";
   let mut str7:&str="World";
   //println!(str6+str7);
   let str8=str6.to_string()+&str7.to_string();
   println!("{}",str8);

  // convert string to str

  let string9:String= "Hello World".to_string();
  let str9:&str = string9.as_str();

}


