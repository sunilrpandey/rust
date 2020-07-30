//extern crate func_lib;
extern crate rand;
use rand::Rng;

extern crate local_lib;

fn main() {

    struct Dummy { a: i32, b: i32 }

    let mut dmy:Dummy = Dummy{a:5,b:7};
    {
        dmy.a = 10;
        dmy.b = 14;
        //dmy = Box::new(Dummy{a:10,b:10});
    }
    dmy.a = 5;
    dmy.b = 7;
    tuple
    
    println!("[a,]")
    //func_lib::second_lib_function();
    let mut rng = rand::thread_rng();
    //let b:bool = rng.gen();
    if true == rng.gen() {
        println!("Hello rand lib!");
    }
    local_lib::func_lib::first_lib_function();    
}
