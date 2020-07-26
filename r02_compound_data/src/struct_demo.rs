#![allow(dead_code)]
// structs demo
use std::mem;

#[derive(Debug)]
struct Point
{
    x:f64,
    y:f64,
    z:f64
}
impl Point {
    fn show(&self) {
        println!("Pos : [{},{},{}]",self.x,self.y,self.z);
    }
}
#[derive(Debug)]
struct Line
{
    start:Point,
    end:Point
}

fn init_point()->Point
{
    Point{x:0.0,y:0.0,z:0.0,}

}
fn from_point(x_coord:f64,y_coord:f64)->Point
{
    Point{x:x_coord,y:y_coord,z:0.0} // no need to write return;
}
pub fn rust_struct_demo() {
    let p1 = init_point();
    let p2 = Box::new(from_point(4.0,5.0));
    println!("Origin on Stack : [{},{},{}]", p1.x, p1.y, p1.z);
    println!("Origin on Heap: [{},{},{}]", p2.x, p2.y, p2.z);

    println!("Sizeof Point on Stack : {}", mem::size_of_val(&p1)); // whole struct is in memeory
    println!("Sizeof Point on Heap: {}", mem::size_of_val(&p2)); // size of pointer

    let p3 = *p2;
    println!("p3 assigned from p2: [{},{},{}]", p3.x, p3.y, p3.z);

    let p4 = Point{x:10.0,y:20.0,z:30.0};
    println!("Init p4 with 10,20,30: [{},{},{}]", p4.x, p4.y, p4.z);

    //udate syntaxes, copy everything except a few from other instance
    let p5 = Point{z:100.0,..p4};
    println!("Using update syntax, should init x,y from p4: [{},{},{}]", p5.x, p5.y, p5.z);

    let line = Line{start:p3,end:p4};
    println!("Line -> [{},{}], [{},{}]", line.start.x,line.start.y,line.end.x,line.end.y);

    //tuple-struct : create struct without named fied
    struct Position(i32,i32,i32);
    let ps = Position(10,100,1000);
    println!("tuple-struct Pos : [{0},{1},{2}]", ps.0,ps.1,ps.2);


    #[derive(Debug)]
    struct Color(i32,i32,i32);
    let color = Color(255,0,0);
    // color = ps; Even though two tuple struct has same type, can not assign to each other
    println!("Color : [{0},{1},{2}]", color.0,color.1,color.2);
    
    println!("{:?}",p5);
    println!("{:?}",line);
    println!("{:?}",color);

    //implement a member function, can have multiple impl block
    let p6 = Point{x:100.0,y:200.0,z:300.0};
    p6.show();
}
