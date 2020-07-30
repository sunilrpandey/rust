# MISC Features

## RUST Library 
### Create Library
Write function in a file (say func_lib.rs)
```rust
    pub fn first_lib_function() -> u8{
        println!("First");
        1
    }
    pub fn second_lib_function() ->u8 {
        println!("Second");
        2
    }
```
Compile it to create lib 
```rust 
rustc --crate-type=lib func_lib.rs
```
Library is created, name would be lib<mod_name>.rlib(i.e. for func_lib, it would be libfunc_lib.rlib) 
Please make sure your functions and mod in lib source file is accessible (pub)

### Use library 
Now got to main.rs and add 
```rust
    extern crate <lib_file_name>;
    extern crate func_lib;

and call required functions in lib
    func_lib::first_lib_function();
```
Now compile main.rs using below command
```rust
rustc main.rs --extern func_lib=libfunc_lib.rlib 
```
I will give you executable which uses lib.
Thats it.

## Use Cargo to create/use lib
### Create lib
 1. Create lib project using

    cargo new --lib <lib_name> //local_lib
    cargo new --lib local_lib

 2. open project/src/lib.rs and write functions to be added. e.g.
 ```rust
    pub fn first_lib_function() -> u8{
        println!("First");
        1
    }
```
 3. build the lib

    cargo build

### Use lib
 1. Go to client/Cargo.toml and add dependencies of just created local libray.
 You can also add downloaded library from crate.io (https://crates.io/) 
 ```rust
    [dependencies]
    rand = "0.7.3" // rand lib from crate.io
    local_lib = {path = "../r04_create_lib"} // library created locally
 ```
 2. Add library reference to client/main.rs
 ```rust 
    extern crate <lib_name>;
    // in our case
    extern crate local_lib;
    extern crate rand;
 ```
  3. Use library functions
  ```rust
    let mut rng = rand::thread_rng();
    local_lib::func_lib::first_lib_function();    
  ```