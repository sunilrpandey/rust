//  unused variable warning can be silenced by prefixing the variable name with an underscore
// When data is bound by the same name immutably, it also freezes. Frozen data can't be modified until the immutable binding goes out of scope
mod struct_demo;
mod enum_demo;
mod some_or_none_demo;
mod string_demo;
mod tuple_struct_demo;
//use enum_demo::Color;
//use enum_demo::*;

fn casting_demo() {
    let fl_num = 66.4341_f32;
    let int_num = fl_num as u8;
    let char_val = int_num as char;
    println!("Casting: {} -> {} -> {}", fl_num, int_num, char_val);

}
// Suppress all warnings from casts which overflow.
#[allow(overflowing_literals)]

fn casting_demo_from_book() {
    // https://doc.rust-lang.org/stable/rust-by-example/types/cast.html
    let decimal = 65.4321_f32;

    // Error! No implicit conversion
    // let integer: u8 = decimal;
    // FIXME ^ Comment out this line

    // Explicit conversion
    let integer = decimal as u8;
    let character = integer as char;

    // Error! There are limitations in conversion rules.
    // A float cannot be directly converted to a char.
    // let character = decimal as char;
    // FIXME ^ Comment out this line

    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    // when casting any value to an unsigned type, T,
    // T::MAX + 1 is added or subtracted until the value
    // fits into the new type

    // 1000 already fits in a u16
    println!("1000 as a u16 is: {}", 1000 as u16);

    // 1000 - 256 - 256 - 256 = 232
    // Under the hood, the first 8 least significant bits (LSB) are kept,
    // while the rest towards the most significant bit (MSB) get truncated.
    println!("1000 as a u8 is : {}", 1000 as u8);
    // -1 + 256 = 255
    println!("  -1 as a u8 is : {}", (-1i8) as u8);

    // For positive numbers, this is the same as the modulus
    println!("1000 mod 256 is : {}", 1000 % 256);

    // When casting to a signed type, the (bitwise) result is the same as
    // first casting to the corresponding unsigned type. If the most significant
    // bit of that value is 1, then the value is negative.

    // Unless it already fits, of course.
    println!(" 128 as a i16 is: {}", 128 as i16);

    // 128 as u8 -> 128, whose value in 8-bit two's complement representation is:
    println!(" 128 as a i8 is : {}", 128 as i8);

    // repeating the example above
    // 1000 as u8 -> 232
    println!("1000 as a u8 is : {}", 1000 as u8);
    // and the value of 232 in 8-bit two's complement representation is -24
    println!(" 232 as a i8 is : {}", 232 as i8);

    // Since Rust 1.45, the `as` keyword performs a *saturating cast*
    // when casting from float to int. If the floating point value exceeds
    // the upper bound or is less than the lower bound, the returned value
    // will be equal to the bound crossed.

    // 300.0 as u8 is 255
    println!(" 300.0 as u8 is : {}", 300.0_f32 as u8);
    // -100.0 as u8 is 0
    println!("-100.0 as u8 is : {}", -100.0_f32 as u8);
    // nan as u8 is 0
    println!("   nan as u8 is : {}", f32::NAN as u8);

    // This behavior incurs a small runtime cost and can be avoided
    // with unsafe methods, however the results might overflow and
    // return **unsound values**. Use these methods wisely:
    unsafe {
        // 300.0 as u8 is 44
        println!(" 300.0 as u8 is : {}", 300.0_f32.to_int_unchecked::<u8>());
        // -100.0 as u8 is 156
        println!("-100.0 as u8 is : {}", (-100.0_f32).to_int_unchecked::<u8>());
        // nan as u8 is 0
        println!("   nan as u8 is : {}", f32::NAN.to_int_unchecked::<u8>());
    }
}

fn size_test(){
    println!("sizeof demo");
    let x = 10;
    println!("size of `x` in bytes: {}", std::mem::size_of_val(&x));    
}
fn main() {
    
    size_test();
    
    struct_demo::rust_struct_demo();
    enum_demo::rust_enum_test();
    some_or_none_demo::options_test();  
    string_demo::string_test();
 
    //casting_demo();
    casting_demo_from_book();
    
    //tuple struct
    tuple_struct_demo::print_demo();

}
