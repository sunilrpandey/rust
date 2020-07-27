#![allow(dead_code)]
use std::fmt;

#[derive(Debug)]
struct TupleStruct(i32);
impl fmt::Display for TupleStruct {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"Using Custom Display : {}", self.0)
    }
}

pub fn print_demo() {
    println!("{} of {:b} people know binary, the other half doesn't", 1, 2);
    println!("{:?}",TupleStruct(3));
    println!("pretty printing : \n{:#?}",TupleStruct(3));
}

struct TupleStructFmtDisplay(i32);
impl fmt::Display for TupleStructFmtDisplay {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"Using Custom Display : {}", self.0)
    }
}

pub fn custom_print_demo() {
    println!("Implement fmt::Display for CustomType");
    println!("Using Custom Print : {}",TupleStructFmtDisplay(3));
}

fn increment(val:i32) -> i32 {
    val + 1
}
pub fn ret_value_without_return_increment() {
    let val = 30;
    println!("incremented value : {}", increment(val));
}


