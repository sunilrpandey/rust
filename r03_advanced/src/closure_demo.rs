#![allow(unused_variables)]
#![allow(unused_imports)]

use std::mem;
pub fn say_hello() {
    println!("Hello Rust!!");
}

fn closure_pass_value_test()
{
    let color = String::from("green");
    
    // closure without arg works
    let cl_print = || println!("Color : {}", color); // will work as taking global value to read
    // let cl_print = |color:String| println!("Color : {}", color); 
    cl_print();

    println!("Locked by closure : {}", color);
    cl_print();
    //----------

    let _reborrow = &color; // copying the reference 
    // let _color_moved = color;       // not allowed (see next cl_print)
    
    // moving the object : not allowed as it is will be used by next refernce of closure function
    // below line will not give error if we remove next cl_print() call
    // off course it will not matter if color is u8/i32 as move does not matter for these types    
    cl_print(); 

    //----------------------- 
    // write a close that modifies this value, here increment_count will take mut& of count
    let mut count = 10; 

    let mut increment_count = || {  // mut is required before increment-count because inside it, it needs mut count& 
        count += 1;
        println!("count : {}", count);
    };
    increment_count();
    
    // AS increment_count IS BEING CALLED AGAIN SO THIS COUNT IS STILL BORROWED, SO YOU CAN NOT USE/BORROW COUNT 
    // IN ANY FUNCTION (println!) OR VARIABLE cp below

    //let cp = count;
    //println!("outside count : {}", count);

    increment_count();
    
    let new_int = Box::new(3);
    let fn_which_use_int = || {
        println!("new int value : {}", new_int);
        mem::drop(new_int);
    };
    fn_which_use_int();
    // fn_which_use_int();

    
    // move before the pipe makes sure that s is moved and destroyed after closure, hence it will not be available for any other use
    // (println!("s is moved : {}", s), if we remove `move` it will be available  
    let s = String::from("Hello Rust");
    let msg_print = move || println!("Message : {}", s);

    msg_print();
    
    // println!("s is moved : {}", s);
    msg_print();

}

fn hello_closure() {
        // use lambda/closure to do mimic online function
        let increment_by_1 = |x:u8| x+1;  //brace is optional for single line statment

        let x = 5;
        println!("{} + 1 = {}", x, increment_by_1(x));
    
        let x = 10;
        let y = 20;
        let add_two_nums = |x,y| x+y;  //brace is optional for single line statment
        println!("{} + {} = {}", x, y, add_two_nums(x,y));
        // -----------------------
        
        let mut factor = 2; 
        {
            let increment_by_2 = |x:u8| -> u8 { 
                let mut z = x;
                z += factor;   
                z
            }; 
    
            println!("{} + {} = {}", x,factor, increment_by_2(x));
        }

        // let borrow_factor = &mut factor; // It will wait factor to be freed, for this we have put incrment by function in scope
    
    
}

fn closure_passby_mut_val() {
    let x = 10;
    let increment_by_4 = |mut x:i32| {
        x += 4;
        println!("Inside closure, x : {}", x);                
    };
    increment_by_4(x);
    println!("Outside closure, x : {}", x);                
}

fn closure_passby_mut_ref() {
    // pass by mut ref
    let increment_by_3 = |x:&mut i32| {
        *x += 3;
        //*x
    };

    let mut x = 10;
    increment_by_3(&mut x);
    println!("Passing by mut ref to increment it by 3, Ans : {}", x );    
}



pub fn closure_test() {
    println!("Demoing Closure!!");

    // call function or assign it to variable and run
    say_hello();
    let sh = say_hello;
    sh();

    hello_closure();

    // Three way to pass arguments
    
    // pass by value 
    closure_pass_value_test();
    // pass by &mut ref
    closure_passby_mut_ref();
    // pass by ref
    closure_passby_mut_val();
    return;
  
}

