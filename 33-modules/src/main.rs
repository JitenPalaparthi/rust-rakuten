mod mymodule;
mod shapes;
mod foo;

use crate::shapes::shape::what_is;
use crate::shapes::rect::rectangle::Rect;

use crate::foo::a::a_foo;

fn main() {
    mymodule::greet();
    let g = mymodule::G{};
    println!("G:{:?}",g);
    mymodule::my_module::greet();
    what_is(); // why not calling the whole path?  becase use is used

    let r1= Rect{l:10.11,b:12.45};

    let a1 = a_foo{};
    println!("a_foo:{:?}",a1);
}
