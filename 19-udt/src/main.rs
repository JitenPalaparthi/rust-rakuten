fn main() {
    let e1: Employee = Employee {
        id: 101,
        name: "Jiten".to_string(),
        address: "Yeshvantpur, Bangalore".to_string(),
    };

    let mut c1: Company = Company {
        refNo: 31231,
        name: "XYZ".to_string(),
        address: Address {
            line1: "house no 101".to_string(),
            city: "Bangalore".to_string(),
            street: "Yeshvantpur street-1".to_string(),
            state: "Karnataka".to_string(),
            country: "India".to_string(),
            pincode: "560086".to_string(),
        },
    };

    println!("Employee object e1:{:?}", e1);
    println!("Company object c1:{:?}", c1);

    let e2: Employee = Employee::new(101, "Ramesh".to_string(), "Chennai".to_string());
    println!("Employee object e2:{:?}", e2);

    let e3: Employee = Employee::new_no_address(102, "Rahim".to_string());

    println!("Employee object c3:{:?}", e3);
}

#[derive(Debug)]
struct Employee {
    id: i32,
    name: String,
    address: String,
}

impl Employee {
    fn new(id: i32, name: String, address: String) -> Self {
        Self {
            id: id,
            name: name,
            address: address,
        }
    }

    fn new_no_address(id: i32, name: String) -> Self {
        Employee {
            id: id,
            name: name,
            address: "".to_string(),
        }
    }

    fn print(&self) {
        println!(
            "id:{:?} name:{:?} address:{:?}",
            self.id, self.name, self.address
        );
    }
}

// &self is used in medhod definition that indicats that the method is using the reference of the object that is calling that method
// Self is a associated function that indicates the func returns an instance of the same type

#[derive(Debug)]
struct Company {
    refNo: i32,
    name: String,
    address: Address, // embedded struct , this is also composition
}

#[derive(Debug)]
struct Address {
    line1: String,
    city: String,
    street: String,
    state: String,
    country: String,
    pincode: String,
}
