fn main() {

    let mut e1:Employee=Employee { id: 101, name: "Jiten", address:"Bangalore", email: "Jitenp@Outlook.Com" };
    println!("Employee e1:{:?}",e1);

    let mut e2 = Employee::new(102,"Rahim","Hyderabad","rahim.md@gmail.com");
    println!("Employee e2:{:?}",e2);
}

#[derive(Debug)]
struct Employee<'a>{
    id:i32,
    name:&'a str, // refenrece
    address:&'a str, //reference
    email:&'a str, //reference
}

// impl Employee<'_>{
//     fn new(id:i32,name:&str,address:&str,email:&str)->Self{
//             Self { id: id, name: name, address: address, email: email }
//     }
// }

impl<'a> Employee<'a>{
    fn new(id:i32,name:&'a str,address:&'a str,email:&'a str)->Employee<'a>{
        Employee{ id: id, name: name, address: address, email: email }
    }
}