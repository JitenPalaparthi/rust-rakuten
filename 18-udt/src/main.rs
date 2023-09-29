fn main() {
    let mut c1:Colour=Colour(10001,"Red".to_string());
    println!("{:?}",c1);
    println!("Id:{:?}",c1.0);
    c1.0 = 10002;
    println!("Id:{:?}",c1.0);
    c1.print();
    let Colour(mut id,mut name) = Colour(22222,"Green".to_string());

    println!("Colour id:{:?}",id);
    id = 23232;
    println!("Colour id:{:?}",id);

}

#[derive(Debug)]
struct Colour(i32,String);

impl Colour{
    fn print(&self){
        println!("Colour id:{:?} name:{:?}",self.0,self.1)
    }
}
