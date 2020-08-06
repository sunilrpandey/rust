mod struct_demo;
mod enum_demo;
mod some_or_none_demo;
mod string_demo;
//use enum_demo::Color;
//use enum_demo::*;

fn casting_demo() {
    let fl_num = 66.4341_f32;
    let int_num = fl_num as u8;
    let char_val = int_num as char;
    println!("Casting: {} -> {} -> {}", fl_num, int_num, char_val);

}
fn main() {
    // struct_demo::rust_struct_demo();
     // enum_demo::rust_enum_test();
    // some_or_none_demo::options_test();  
    // string_demo::string_test();
 
    casting_demo();

}
