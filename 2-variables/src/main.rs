
static num3:i64=1232131231;

fn main() {
    
    let num1 = 100; // a simple way of declaring a variable 

    let num2:i8=126;

    let float1:f32=12.34;

    let ok:bool= true;

    let mybox:Box<i32>=Box::new(102); // allocatiing memory in the heap

    let char1:char='𫚭';
    hello(100);
    // let float2 = 12.34;

    //  let float3 = 12.34;

    println!("num1:{} num2:{} float1:{} ok:{} char1:{}",num1,num2,float1,ok,char1);
    println!("Global static varialbe:{}",num3);
    println!("Healp allocated smart box:{}",mybox);

}

fn hello(num:i32){
println!("num is {}",num);
}

// numeric datatypes
// i8,i16,i32,i64,i128,isize
// u8,u16,u32,u64,u128,usize

// f32, f64

// IGB --> 1073741824 bytes

// The whole memory that is given by your OS for a process is divided into 

// Code/Text Segment -> Readonly memory
// Data Segment
    // UDS -> Uninitilies global varialbes are stored here
    // IDA -> Globaal varialbes, Static Variables, Constants are stored here
// Stack Memory: Local varialbe, Function stack frames , defined types are stored here
// Heap Memory:  Growable , Shrinkable data types which we cannot decide at compile time.. are stored in heap memory
                // malloc, calloc, jalloc
