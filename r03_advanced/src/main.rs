mod closure_demo;
use closure_demo::*;
mod hof_demo;
mod traits_demo;
mod ownership_borrowing_demo;

fn main() {

    ownership_borrowing_demo::ownership_borrowing_test();
    closure_test();
    traits_demo::traits_test();    
    hof_demo::high_order_functions_test();
}
