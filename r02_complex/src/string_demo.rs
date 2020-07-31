#![allow(dead_code)]

fn slice_int(slice:&[i32]){
    println!("slice length : {}", slice.len());
}
fn change_element(slice:&mut[i32]){
    slice[0] = 100;
    println!("slice -> {:?}", slice);
}

pub fn slice_array_int_test() {
    let mut a = [1,2,3,4,5,6,7];
    slice_int(&a[1..4]);
    change_element(&mut a[1..4]);
    println!("array -> {:?}", a);
}

fn iterate_through_const_string() {
    let qt:&'static str = "Honesty is the best policy";
    println!("{}",qt);

    for word in qt.split_whitespace() {
        println!("{}",word);
    }

}

pub fn string_test() {

    slice_array_int_test();
    iterate_through_const_string();    
    
}