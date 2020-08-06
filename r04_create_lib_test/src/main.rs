//extern crate func_lib;
extern crate rand;
use rand::Rng;

extern crate local_lib;

fn main() {

    //func_lib::second_lib_function();
    let mut rng = rand::thread_rng();
    //let b:bool = rng.gen();
    if true == rng.gen() {
        println!("Hello rand lib!");
    }
    local_lib::func_lib::first_lib_function();    
}
