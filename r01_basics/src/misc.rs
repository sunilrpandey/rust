#![allow(dead_code)]
use std::fmt;



fn increment(val:i32) -> i32 {
    val + 1
}
pub fn ret_value_without_return_increment() {
    let val = 30;
    println!("incremented value : {}", increment(val));
}


