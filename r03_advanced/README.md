# RUST Advanced Features

- [ ] [Closure](#closure)
- [ ] [High Order Functions](#hof)
- [ ] [Traits](#traits)
    - [Generic Traits](#generic)
- [ ] [Ownership & Moves](#ownership)
- [ ] [Pass reference](#passref)
- [ ] [Pass mut reference](#passmutref)

## <a name=closure>Closure</a>
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
## <a name=hof>High Order Functions</a>
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
## <n name=traits>Traits</a>
A trait is a collection of methods defined for an unknown type: Self. They can access other methods declared in the same trait.
Traits are very similar to concept of interface in C++. Traits can be implemented for any data type
For example I define few methods for Shape trait
```rust
trait Shape
    {
        fn create(in_name:&'static str) -> Self;
        fn name(&self)->&'static str;
        fn render(&self) {
            println!("{} abstract shape, can't render",self.name());
        } 
    }
```
Now we have Shape such as Square(as struct) and we implement shape traits for Square
```rust
    impl Shape for Square 
    {
        fn create(in_name:&'static str) -> Square
        {
            Square{name:in_name}
        }
        fn name(&self)->&'static str {
            self.name
        }
        fn render(&self) {
            println!("{} rendered.",self.name());
        }
    }
    struct Square
    {
        name:&'static str
    }
```
and here how we can use Square
```rust
    let shape:Square = Square{name:"My Square"}; OR
    let shape:Square = Shape::create("My Square");
    shape.render();    
```
### <a name=generic>Generic Traits</a>
```rust 
    trait Summable<T>
    {
        fn sum(&self) -> T;
    }
```
Now we apply above trait to a Vector
```rust 
    impl Summable<i32> for Vec<i32>
    {
        fn sum(&self) -> i32 {
            let mut result = 0;
            for i in self {
                result += *i;
            }
            return result;
        }
    }
```
Now Let us test methods from trait to check Vec
```rust
    let v = vec![30,20,10];
    println!("Sum of elements : {}", v.sum());
```

## <a name=ownership>Ownership & Moves</a>
Rust follows move semantics to maintain ownership of the storage. However For basic integral type it follow copy traits instead of move.
For example, take and example of vector
```rust
    let v1 = vec![1,2,3];
    println!("v1 = {:?}",v1);
    let v2 = v1;
    println!("v2 = {:?}",v2);
    
Now below line will give error since v1 no longer owns the storage for vector
    //println!("v1 = {:?}",v1);
``` 
Let us what happens with integral types
```rust
    // Let us see what happens with integral type
    let i = 32;
    let j = i;
    println!("Here both exists i = {} & j = {}", i, j);

    //Now let us explicitly assign memory to it
    let i = Box::new(32);
    let j = i;
    //println!("Wont compile as value of i is borrowed  i = {} & j = {}", *i, *j);
```
May be we can return from a function to which we passed ownership
```rust
    fn show_vector(v:Vec<i32>) -> Vec<i32>{
        println!("show vec : {:?}",v);
        v
    }

```
And here is how vector can be passed and collected
```rust 
    fn borrow_and_return_test() {

        let v = vec![12,23,45];
        let v = show_vector(v);
        println!("returned vec : {:?}",v);

    }
```
### <a name=passref>Pass reference </a>
Passing and returing is pain, so we pass by reference, Please not you can not modify vec ref in show_vec_ref function
```rust
    fn show_vector_ref(v:&Vec<i32>){
        println!("vec (ref): {:?}",v);
        //v.push(100);      //Not possibe, ref is not mutable
    }
```
..and here is how we pass and collect
```rust
    fn pass_by_ref_test() {

        let v = vec![12,23,45];
        show_vector_ref(&v);
        println!("returned vec (ref) : {:?}",v);

    }
```
### <a name=passmutref>Pass mut reference</a>
To pass ref as mutable write mut after `&`
```rust
    fn show_vector_mut_ref(v:&mut Vec<i32>){
        println!("vec (mut ref): {:?}",v);
        v.push(100);
    }

    fn pass_by_mut_ref_test() {

        let mut v = vec![12,23,45];
        show_vector_mut_ref(&mut v);
        println!("returned vec (mut ref) : {:?}",v);
    }
```