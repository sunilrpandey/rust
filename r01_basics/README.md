# Data Types
## Basic Data types

* Rust datatype are similar to i8, u16, i64, u128 etc
* Varibales are read only by default, use `mut` if you want to modify
* specifying data type is optional, compiler understands it interpret it based on its usage
* var:isize refer to any integer type
* few utility functions and hwo we  
* for const use `const MAX_VAL: u32 = 10_000;` datatype is necessary, _  can be used for readability

## Tuples
contains multiple values, access it using .0,.1 etc
```
    let tup: (i32, f64, u8) = (100, 3.4, 10);
    let tar = tup;
    let (x, y, z) = tup;
    println!("x = {}, y = {} , z = {}", x,y,z);
    println!("x = {}, y = {} , z = {}", tar.0,tar.1,tar.2);
```

## Array
collection of data of same data types
```
    let names = ["Tom", "Dick", "Harrt"];
    println!("Tom : {}", names[0]);

    // array of 5 int
    let arr_of_int:[i32;5] = [10, 20, 30, 40, 50];
    println!("last element : {}", arr_of_int[4]);

    let arr_of_3 = [3;5];
    println!("first element : {}", arr_of_3[0]);
```

## Use Functions
Function starts with keyword `fn` 
### local function 
Write function locally and use it. 
```
    fn local_increment_func(mut x:u8) -> u8{x = x + 1; return x;}
    let mut var:u8 = 10;
    var = local_increment_func(var);    
```
### function in another module/file
- Write function in another file(function_demo.rs, in this case), make it public if required.
- Add its reference in file using this 
    > mod <file_name>; // mod function_demo; here
- use this function using file_name::func_name // function_demo::increment here
- use `use` statement to add function in current scope

## Any Control : Yes, most of you 
### IF Statement
- () after if optional
- if, else and multiple if/else similar to C++ etc supported
- non zero value does not evaluate to bool, gives compilation error 
- if and else blocks should return same data types else compiler
```
let number = if true { 1 } else { 0 }; //works
let number = if true { 1 } else { "false" }; //wont work

```

### Loop for infinite loop
```
    loop{
            println!("Infinite loop : press Ctrl + C to quit");
        }
```
- Return value from loop on some contion, but return value using break, not return

### While loop : similar to C++ etc
No different from other language
```
    let mut i = 0;
    while i < 5 {
        println!("{}!", i);
        i += 1;
    }
```
### For loop :
Iterate through content
```
    let a = [100,200,300,400,500];
    for e in a.iter() {
        print!("{}! ", e);
    }
```
Iterate using container index
```
    let sz = a.len();
    for i in 0..sz {
        print!("{}! ", a[i]);
    }
```
User rev() for reverse indexing
```
    for i in (0..sz).rev() {
        print!("{}! ", a[i]);
    }
```
## Comments Please??
- Yes similar to C++ single line - // and multiline using /* */ 

## Few functions we use very often
    * mem::size_of_val(&var)
    * var = 2 + 4 + 7 or 2|4 or 3<<5>>
    * i32::pow(a,3) or f64::powf(2.5,std::f64::consts::PI)
    
## Reference 
[file/module discusstion](https://stackoverflow.com/questions/27613874/how-do-i-tell-cargo-to-build-files-other-than-main-rs)