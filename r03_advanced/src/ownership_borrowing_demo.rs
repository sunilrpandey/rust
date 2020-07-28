
fn lambdatest()
{
    let v = vec![10,20,30];
    let foo = |v:Vec<i32>|();
    foo(v);
    
    // wont compile as ownership of v is moved to foo(v) 
    //println!("v = {:?}",v);
}
fn show_vector(v:Vec<i32>) -> Vec<i32>{
    println!("show vec : {:?}",v);
    v
}

fn borrow_and_return_test() {

    let v = vec![12,23,45];
    let v = show_vector(v);
    println!("returned vec : {:?}",v);

}
fn show_vector_ref(v:&Vec<i32>){
    println!("vec (ref): {:?}",v);
    
    // You can not modify here as ref of v is not mutable
    //v.push(100); 
}

fn pass_by_ref_test() {

    let v = vec![12,23,45];
    show_vector_ref(&v);
    println!("returned vec (ref) : {:?}",v);

}

fn show_vector_mut_ref(v:&mut Vec<i32>){
    println!("vec (mut ref): {:?}",v);
    v.push(100);
}

fn pass_by_mut_ref_test() {

    let mut v = vec![12,23,45];
    show_vector_mut_ref(&mut v);
    println!("returned vec (mut ref) : {:?}",v);

}
pub fn ownership_borrowing_test()
{
    pass_by_ref_test();
    pass_by_mut_ref_test();
    return;
    borrow_and_return_test();
    lambdatest();
    let v = vec![1,2,3];    
    println!("v = {:?}",v);
    let v2 = v;
    println!("v = {:?}",v2);
    //println!("v = {:?}",v);

    // Let us see what happens with integral type
    let i = 32;
    let j = i;
    println!("Here both exists i = {} & j = {}", i, j);

    //Now let us explicitly assign memory to it
    let i = Box::new(32);
    let j = i;
    //println!("Wont compile as value of i is borrowed  i = {} & j = {}", *i, *j);
 
}