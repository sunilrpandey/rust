# Data Types
- [Basic Types](#datatype)
- [Tuples](#tuple)
- [Array](#array)
- [Functions](#functions)
- [Conditional Controls](#controls)
    - [If/Else Statment](#if)
    - [Match Statement](match)
- [Iterative Controls](#looping)
    - [Loop](#loop)
    - [While loop](#while)
    - [For Loop](#for)
- [Comments](#comments)

- [Utility functions we use very often](#extra)
- [References](#reference)
## <a name=datatype>Basic Data types

* Rust datatype are similar to i8, u16, i64, u128 etc
* Varibales are read only by default, use `mut` if you want to modify
* specifying data type is optional, compiler understands it interpret it based on its usage
* var:isize refer to any integer type
* few utility functions and hwo we  
* for const use `const MAX_VAL: u32 = 10_000;` datatype is necessary, _  can be used for readability

## <a name=tuple>Tuples</a>
contains multiple values, access it using .0,.1 etc
```rust
    let tup: (i32, f64, u8) = (100, 3.4, 10);
    let tar = tup;
    let (x, y, z) = tup;
    println!("x = {}, y = {} , z = {}", x,y,z);
    println!("x = {}, y = {} , z = {}", tar.0,tar.1,tar.2);
```

## <a name=array>Array</a>
collection of data of same data types
```rust
    let names = ["Tom", "Dick", "Harrt"];
    println!("Tom : {}", names[0]);

    // array of 5 int
    let arr_of_int:[i32;5] = [10, 20, 30, 40, 50];
    println!("last element : {}", arr_of_int[4]);

    let arr_of_3 = [3;5];
    println!("first element : {}", arr_of_3[0]);
```

## <a name=functions>Functions</a>
Function starts with keyword `fn` 
### local function 
Write function locally and use it. 
```rust
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

## <a name=controls>Conditional Controls</a>
### <a name=if>IF Statement</a>
- () after if is optional
- if, else and multiple if/else similar to C++ etc supported
- non zero value does not evaluate to bool, gives compilation error 
- if and else blocks should return same data types else compiler
```rust
let number = if true { 1 } else { 0 }; //works
let number = if true { 1 } else { "false" }; //wont work
```
- No need to return something from if/else to get back result, here returning string to print
```rust
    println!("Print using if/else : {}", if true {"True"} else {"False"});
```
### <a name=match>Match Statement</a>
Its similar to switch in C++ but syntaxes are little different. Please note no semicolon after options and closing brace for match
```rust
    num = 6;
    let num_str = match num {
        1 => "One",
        2 => "Two",
        5 => "Five",
        1...10 => "Not Found!!",
        _ => "InValid Input",
    };
    println!("spell of {} : {}", num, num_str);        
```
## <a name=looping>Iteration</a>
### <a name=loop>Loop for infinite iteration</a>
```rust
    loop{
            println!("Infinite loop : press Ctrl + C to quit");
        }
```
- Return value from loop on some contion, but return value using break, not return

### <a name=while>While loop : similar to C++ etc</a>
Not much different from other languages
```rust
    let mut i = 0;
    while i < 5 {
        println!("{}!", i);
        i += 1;
    }
```
### <a name=for>For loop : The simple and sweet loop</a>
Iterate through content
```rust
    let a = [100,200,300,400,500];
    for e in a.iter() {
        print!("{}! ", e);
    }
```
Iterate using container index, it can be any [start..end)
```rust
    let sz = a.len();
    for i in 0..sz {
        print!("{}! ", a[i]);
    }
```
Use rev() for reverse indexing
```rust
    for i in (0..sz).rev() {
        print!("{}! ", a[i]);
    }
```
User enumerate function to iterate over range
```rust
for (i, x) in (25..31).rev().enumerate() {
        println!(" pos[{}] = [{}]", i, x);
    }
```

## <a name=comments>Any Comments</a>
- Yes similar to C++ single line - // and multiline using /* */ 

## <a name=extra>Few functions we use very often</a>
    * mem::size_of_val(&var)
    * var = 2 + 4 + 7 or 2|4 or 3<<5>>
    * i32::pow(a,3) or f64::powf(2.5,std::f64::consts::PI)
    
## <a name=reference>References</a>
[file/module discusstion](https://stackoverflow.com/questions/27613874/how-do-i-tell-cargo-to-build-files-other-than-main-rs)
[The Rust Book](https://doc.rust-lang.org/book/ch03-00-common-programming-concepts.html)