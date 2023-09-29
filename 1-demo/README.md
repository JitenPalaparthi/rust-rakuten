- Rust does not have garbage collector  
- Does rust has runtime? Yes a minimalistic runtime
- The code that is used at runtime is generated/included by the compiler to the rust binary

- Run rust applicaion

```
cargo run 
```
- Compile and generate binary using ructc tool
```
rustc src/main.rs
```

- Compile and generate named binary using rustc tool

```
rustc -o demo src/main.rs
```

- To save LLVM IR code from the rust code 

```
rustc --emit=llvm-ir src/main.rs 
```