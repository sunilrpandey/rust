#![allow(unused_variables)]
#![allow(unused_imports)]
// Find the sum of all the squared odd numbers under 1000

fn is_odd(x:i32) -> bool {
    x % 2 != 0
}
fn sum_of_sqrd_odd_numbers_under_1000() -> i32{

    let mut sum = 0;
    for x in 1.. {
        let sqrd = x * x;
        if sqrd >= 1000 {
            break;
        } else if is_odd(sqrd) {
            sum  += sqrd;       
        }
    }

    sum
}
pub fn high_order_functions_test() {
    //usign traditional function call
    let classic_sum = sum_of_sqrd_odd_numbers_under_1000();   
    println!("Classic sum result : {}", classic_sum);

    let hof_sum = (1..).map(|x| x*x)                        // iterate through numbers and get squared
                        .take_while(|&sqrd| sqrd < 1000)    // check if sqrd value is less than given number i.e. 1000
                        .filter(|&sqrd| is_odd(sqrd))       // check if sqrd nubmer is odd
                        .fold(0,|sum,sqrd| sum + sqrd);     // accumulate the sum
    
    println!("HOF sum result : {}", hof_sum);
}   