pub mod func_lib {
        pub fn first_lib_function() -> u8
        {
            println!("First");
            1
        }

        pub fn second_lib_function() ->u8 
        {
            println!("Second");
            2
        }
}
#[test]
fn lib_test_first() {
    assert_eq!(1, func_lib::first_lib_function());
}

