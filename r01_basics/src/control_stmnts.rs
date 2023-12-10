fn controls_if_demo() {
    let number = 7;
    // if number { println!("Compilation error");}   // will give error  , no convertio to bool
    if number == 7 {  // no error  , convertio to bool
        println!("Equal to 7");
    } else {
        println!("Not Equal to 7");
    }  

    println!("Print using if/else : {}", if true {"True"} else {"False"});

}

fn controls_loop_demo() {

    fn invoke_infinte_loop() {
        loop{
            println!("Infinite loop : press Ctrl + C to quit");
        }
    }

    // invoke_infinte_loop();
    // return value from loop, but return value using break, not return 
    let mut cnt = 0;
    let ret_from_loop = loop {
        cnt += 1;
        if cnt == 10 {
            break cnt;
        } 
    };
    println!("ret from loop : {}", ret_from_loop);
}

fn controls_while_loop_demo() {

    // while loop demo, loop through until some condition or loop through some container
    println!("While loop demo");
    
    let mut i = 0;
    while i < 5 {
        println!("{}!", i);
        i += 1;
    }
}

fn controls_match_demo() {

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
    let num_str: &str = match num {
        1 => "One",
        2 => "Two",
        5 => "Five",
        1...10 => "Not Found!!",
        _ => "InValid Input",
    };
    println!("spell of {} : {}", num, num_str);    

}

fn controls_for_loop_demo() {

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
}


pub fn control_statement_demo()
{
        //control demos
        controls_if_demo();    
        controls_loop_demo();     
        controls_while_loop_demo(); 
        controls_for_loop_demo(); 
        controls_match_demo(); 
}