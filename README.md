# rust
Exploring rust

## Few Imp Points
## println
Printing is handled by a series of macros defined in std::fmt some of which include:
* println! is a macro that prints text to the console.
* print is same except it does not append new line
* eprint!/eprintln! are same as print! version except it outputs to io::stderr instead of io::stdout
* println can have positional arguments
println!("{0}, this is {1}. {1}, this is {0}", "first", "second");
* println can also use named arguments
* println!("{subject} {verb} {object}",
             object="the best policy",
             subject="Honesty",
             verb="is");
* {:b} used for binary representation
* To print custom data type add `#[derive(Debug)]` just before struct type and print usign "{:?}" or pretty print using "{:#?}" 
* or implemnt fmt::Display for Custom Types
```rust
impl fmt::Display for TupleStructFmtDisplay {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"Using Custom Display : {}", self.0)
    }
}
``` 
## Literals
* x:i32  can be assigned 3 as 
```rust
let x = 3i32
let y = 2u32;
```
## Others
### No need to return value, just dont put `;` after value to return
```rust
fn increment(val:i32) -> i32 {
    val + 1
}
```
### Type is used for aliasing, similar to `using` in C++11 
```rust 
type Operations = VeryVerboseEnumOfThingsToDoWithNumbers; 
type Price = u64;
```
### Static keyword
static: A possibly mutable variable with 'static lifetime. The static lifetime is inferred and does not have to be specified. Accessing or modifying a mutable static variable is unsafe.
```rust
static LANGUAGE: &str = "Rust";
```
### Unused variable bindings; these warnings can be silenced by prefixing the variable name with an underscore

### Anytype to string
```
let i = 5;
let five = String::from("5");

assert_eq!(five, i.to_string());
```
## Casting
No implicit conversion in RUST, use `as` for explicit conversion wherever allowed. e.g A float cannot be directly converted to a char, if required convert it to int then char 
```rust
let fl_num = 66.4341_f32;
let int_num = fl_num as u8;
let char_val = int_num as char;
println!("Casting: {} -> {} -> {}", fl_num, int_num, char_val);
```

## Constructs to silent warning
#![allow(unused_variables)]\
#![allow(unused_imports)]\
#[allow(non_camel_case_types)]\
#![allow(overflowing_literals)]\

## Reference:
[Rust by Example](https://doc.rust-lang.org/stable/rust-by-example/index.html)

