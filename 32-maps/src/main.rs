use std::collections::HashMap;
fn main() {
    let mut map1:HashMap<i32,String>= HashMap::new();

    map1.insert(560086, "Bangalore,Yeshvantpur".to_string());
    map1.insert(560096, "Bangalore,Rajajinagar".to_string());
    map1.insert(560034, "Bangalore,Koramangala".to_string());
    map1.insert(560046, "Bangalore,Whitefiled".to_string());

    let v= map1.get(&560076);

    
    match v{
        Some(v1)=>println!("{:?}",v1),
        None=>println!("None value for the given key")
    } 

    for (k,v) in &map1{
        println!("Key: {} Value: {}",k,v);
    }


}
