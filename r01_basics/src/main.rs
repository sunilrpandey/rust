
// Implementing the fmt::Display trait automatically implements the ToString trait which allows us to convert the type to String.
// #[allow(dead_code)] is an attribute which only apply to the module after it.

mod misc;
mod function_demo;
use function_demo::increment;
mod control_stmnts;

fn out_function_inc_by2(mut x:u8) -> u8{
    println!("outside function");
    x = x + 2;
    return x;
}

fn basic_datatype_demo() {
    //data types declaration;
    let x:u8 = 10;
    let mut y = 0;
    y += 5;
    println!("x = {} & y = {}",x,y);

    let num = "42";
    println!("num from str : {}", num);

}
fn basic_expression_demo () {
    let y = 3 + 5 + 6;
    println!("y = {} & is y equals to 14 : {}",y, (y == 14));
    println!("3^4 = {}",i32::pow(3,4));
    println!("3.5^pi = {}",f64::powf(3.5,std::f64::consts::PI));
    println!("x = {x}",x = 18);
}

fn tuple_demo() {
    
    let tup: (i32, f64, u8) = (100, 3.4, 10);
    let tar = tup;
    let (x, y, z) = tup; // destructuring
    println!("x = {}, y = {} , z = {}", x,y,z); 
    println!("x = {}, y = {} , z = {}", tar.0,tar.1,tar.2);
}
fn basic_shadowing_demo() {

    let s = "const string";
    let s = s.len();
    println!("length of s = {}",s);
}

fn array_demo() {
    //Array of strings
    let names = ["Tom", "Dick", "Harrt"];
    println!("Tom : {}", names[0]);

    // array of 5 int
    let arr_of_int:[i32;5] = [10, 20, 30, 40, 50];
    println!("last element : {}", arr_of_int[4]);

    // all entries will have 5
    let arr_of_3 = [3;5];
    println!("first element : {}", arr_of_3[0]);
}


fn basic_functions_demo() {
    // How to use function 
    fn local_increment_func(mut x:u8) -> u8{x = x + 1; return x;}
    let mut var:u8 = 10;
    var = local_increment_func(var);
    println!("incremeted var using inline func, var: {}", var);
    var = function_demo::increment(var);
    println!("incremeted var using diff module/file, var : {}", var);
    println!("Brought function to current scope var : {}", increment(var)); // using use function_demo::increment;

    //call function outside this function
    println!("Call outside function to inc by 2, var : {}", out_function_inc_by2(var));

}

fn reference_demo()
{
    let mut s = String::from("hello");
    println!("s = {}",s);

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    
    println!("{} and {}", r1, r2);    // variables r1 and r2 will not be used after this point
    
    let r3 = & mut s; // no problem
    r3.push_str("world");
    println!("{}", r3);
    //println!("{} = {}", r3,s); wnt work since two refernce (mutable + ) are in action
    println!("s = {}",s); // will work since r3 reference is already done, will not work if print for r3 is put after this line
    //println!("{}", r3);
    
}

fn no_dangle() -> String {
    let s = String::from("hello");

    s
    // &s // not allowed
}
fn dangling_reference_demo()
{
    // why below code works.. how returned string is managed if it is not being collected
    no_dangle();
    println!("s = {}","hello");    

    let s = no_dangle();
    println!("s = {}",s);    

}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn slice_demo(){

    //slice is always a reference
    let s = String::from("Honesty is best policy");
    let f_word = first_word(&s);

    println!("First word of the quote : {}", f_word)


}

fn main() {

    println!("hahaha!");
    println!("Welcome to Rust!!");
    reference_demo();
    dangling_reference_demo();
    slice_demo();
    misc::ret_value_without_return_increment();
    
    basic_functions_demo();
    basic_datatype_demo();
    basic_expression_demo();
    basic_shadowing_demo();

    array_demo();
    tuple_demo();    
    control_stmnts::control_statement_demo();

    return;
    

 }
