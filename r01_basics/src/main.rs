mod function_demo;
use function_demo::increment;

fn out_function_inc_by2(mut x:u8) -> u8{
    println!("outside function");
    x = x + 2;
    return x;
}

fn main() {
    //data types declaration;
    let x:u8 = 10;
    let mut y = 0;
    y += 5;
    println!("x = {} & y = {}",x,y);

    // expresessons
    let y = 3 + 5 + 6;
    println!("y = {} & is y equals to 14 : {}",y, (y == 14));
    println!("3^4 = {}",i32::pow(3,4));
    println!("3.5^pi = {}",f64::powf(3.5,std::f64::consts::PI));

    let num = "42";
    println!("num from str : {}", num);

    // Shadowing
    let s = "const string";
    let s = s.len();
    println!("length of s = {}",s);

    //== Tuples 
    let tup: (i32, f64, u8) = (100, 3.4, 10);
    let tar = tup;
    let (x, y, z) = tup;
    println!("x = {}, y = {} , z = {}", x,y,z); 
    println!("x = {}, y = {} , z = {}", tar.0,tar.1,tar.2);

   //Array of strings
    let names = ["Tom", "Dick", "Harrt"];
    println!("Tom : {}", names[0]);

    // array of 5 int
    let arr_of_int:[i32;5] = [10, 20, 30, 40, 50];
    println!("last element : {}", arr_of_int[4]);

    let arr_of_3 = [3;5];
    println!("first element : {}", arr_of_3[0]);

    // How to use function 
    fn local_increment_func(mut x:u8) -> u8{x = x + 1; return x;}
    let mut var:u8 = 10;
    var = local_increment_func(var);
    println!("incremeted var using inline func, var: {}", var);
    var = function_demo::increment(var);
    println!("incremeted var using diff module/file, var : {}", var);
    println!("Brought function to current scope var : {}", increment(var));

    //call function outside this function
    println!("Call outside function to inc by 2, var : {}", out_function_inc_by2(var));

    println!("\n\n =============== IF DEMO ============");

    let number = 7;
    // if number { println!("Compilation error");}   // will give error  , no convertio to bool
    if number == 7 {  // no error  , convertio to bool
        println!("Equal to 7");
    } else {
        println!("Not Equal to 7");
    }  

    println!("Print using if/else : {}", if true {"True"} else {"False"});

    println!("\n\n =============== LOOPS(loop/for/while) DEMO ============");
    fn invoke_infinte_loop() {
        loop{
            println!("Infinite loop : press Ctrl + C to quit");
        }
    }
    // invoke_infinte_loop();

    // return value from loop, but return value using break, not return 
    let mut cnt = 0;
    let ret_from_loop = loop{
        cnt += 1;
        if cnt == 10 {
            break cnt;
        } 
    };
    println!("ret from loop : {}", ret_from_loop);

    // while loop demo, loop through until some condition or loop through some container
    println!(" while loop demo");
    let mut i = 0;
    while i < 5 {
        println!("{}!", i);
        i += 1;
    }

    // for loop, loop iterate elements through container
    println!("For loop demo");
    println!("Loop thrugh element");
    let a = [100,200,300,400,500];
    for e in a.iter() {
        print!("{}! ", e);
    }
    println!("Loop thrugh array index");
    let sz = a.len();
    for i in 0..sz { // it can be any start..end
        print!("{}! ", a[i]);
    }
    println!("print in reverse :");
    for i in (0..sz).rev() {
        print!("{}! ", a[i]);
    }

    println!("\nUser enumerate function to iterate over range");
    for (i, x) in (25..31).rev().enumerate() {
        println!(" pos[{}] = [{}]", i, x);
    }
    println!();

    println!("Match demo");
    let mut num = 2;
    match num {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        _ => println!("Any Default"),
    }
    // can collect values using match as well
    num = 6;
    let num_str = match num {
        1 => "One",
        2 => "Two",
        5 => "Five",
        1...10 => "Not Found!!",
        _ => "InValid Input",
    };
    println!("spell of {} : {}", num, num_str);    
    
 }
