# RUST Advanced Features

## Closure
also called lambda, good for defining on the fly functions, input variables are taken in || insetead of {}
input variables, return types are inferred
```rust
let increment_by_1 = |x:u8| x+1; //brace is optional for single line statment
let add_two_nums = |x,y| x+y;  // providing data tyes of input variable is optional

let ret_int_val = || 2; // return type is optional
```
Closure can capture by 
- by reference: &T
- by mutable reference: &mut T
- by value: T

If captured value is immutable you can use it inside closure and outside, you can take reference of this but can not move.
```rust
    let color = String::from("green");
    let cl_print = || println!("Color : {}", color); 
    
    cl_print();
    let _reborrow = &color; // copying the reference ALLOWED

    //let _color_moved = color;     // NOT ALLOWED
    cl_print();
```
When captured value in closure is mutable, make closure mutable by writing mut before closure name
```rust 
    let mut count = 10; 
    let mut increment_count = || {  // mut is required before increment-count because inside it, it needs mut count& 
        count += 1;
        println!("count : {}", count);
    };
    increment_count();
```
As count(the mutable variable) is already borrowed by the closure, it can not be borrowed by any other function or expression untill all 
closure are not called, such as 
```rust
    let cp = count; OR
    println!("outside count : {}", count);

    ...
    ...
    increment_count();
```
Writing `mem::drop(new_int)` in closure makes sure that movable object is destroyed once it is called
```rust
    let new_int = Box::new(3);
    let fn_which_use_int = || {
        println!("new int value : {}", new_int);
        mem::drop(new_int);
    };
    fn_which_use_int();
    // fn_which_use_int();
```
`move` before the pipe makes sure that s is moved and destroyed after closure, hence it will not be available for any other use  (println!("s is moved : {}", s), if we remove `move` it will be available  
```rust
    let s = String::from("Hello Rust");
    let msg_print = move || println!("Message : {}", s);

    msg_print();
    // 
    //println!("s is moved : {}", s);
    msg_print();
```
## High Order Functions
Takes two or more functions and provide more useful functionality.
Let us take an example, suppose we have to find the sum of all the squared odd numbers under 1000 
Here is the classic approch
```rust
fn is_odd(x:i32) -> bool {
    x % 2 != 0
}
fn sum_of_sqrd_odd_numbers_under_1000() -> i32{

    let mut sum = 0;
    for x in 1.. {
        let sqrd = x * x;
        if sqrd >= 1000 {
            break;
        } else if is_odd(sqrd) {
            sum  += sqrd;       
        }
    }

    sum
}
```
.. and here is the HOF way
```rust
    pub fn high_order_functions_test() {
    //usign traditional function call
    let classic_sum = sum_of_sqrd_odd_numbers_under_1000();   
    println!("Classic sum result : {}", classic_sum);

    let hof_sum = (1..).map(|x| x*x)                        // iterate through numbers and get squared
                        .take_while(|&sqrd| sqrd < 1000)    // check if sqrd value is less than given number i.e. 1000
                        .filter(|&sqrd| is_odd(sqrd))       // check if sqrd nubmer is odd
                        .fold(0,|sum,sqrd| sum + sqrd);     // accumulate the sum
    
    println!("HOF sum result : {}", hof_sum);
} 
```