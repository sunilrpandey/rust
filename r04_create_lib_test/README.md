# MISC Features

- [ ] [Rust Lib](#rustlib)
    - [ ] [Create Lib](#create)
    - [ ] [Use Lib](#use)
- [ ] [Use Cargo the Package Manager](#libcargo)
    - [ ] [Import Lib](#importcargo)
    - [ ] [Create Lib](#createcargo)
    - [ ] [Use Lib](#usecargo)


## <a name=rustlib>RUST Library </a>
### <a name=create>Create Library</a>
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

### <a name=use>Use library </a>
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

## <a name=libcargo>Use Cargo to create/use lib</a>
### <a name=createcargo>Create lib</a>
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
### <a name=importcargo>Import lib</a>
- [ ] Download library from crate.io (https://crates.io/) 
- [ ] Upadate dependencies in Cargo.toml by adding lib and its version
    ```rust
    [dependencies]
    rand = "0.7.3" // rand lib from crate.io
    ```

### <a name=usecargo>Use lib</a>
 1. Update dependencies in Client/Cargo.toml by adding just created local libray and its relative path
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