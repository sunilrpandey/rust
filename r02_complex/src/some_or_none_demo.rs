
fn division_by_zero(nr:f64, dr:f64) {
    //let res:Option<f64> = Some(nr/dr);
    let res = if dr != 0.0 { Some(nr/dr)} else {None};
    match res {
        Some(z) => println!("{}/{} = {}",nr,dr,z),
        None => println!("Division not possible if dr is zero")
    }
}

fn iflet_whilelet_demo() {
    println!("Demo : Vector with while-let");
    let mut a = vec![1,2,3];
    while let Some(x) = a.pop() {
        println!("{}",x);
    }
    if let None = a.pop() {
        println!("all done");
    } else {
        println!("something left");
    }

}

pub fn options_test()
{
    let nr = 3.0;
    let dr = 0.0;

    division_by_zero(nr,dr);    

    iflet_whilelet_demo();

    
}

