#![allow(dead_code)]
// structs demo
use std::mem;
use std::convert::From;

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

// Converts float value to Point
impl From<f64> for Point {
    fn from (val:f64) ->Self {
        Point{x:val, y:val, z:val}

    }
}
impl Into<f64> for Point {
    fn into (self) ->f64 {
        self.x
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
#[derive(Debug)]
struct Person<'a> {
    name: &'a str, // The 'a defines a lifetime        
    age: u8,
}

fn init_struct_demo() {

    /*
    let name = "Joker";
    let age = 40;
    let person = Person{ name,age};
    //println!("name : {}, age : {}",person.name, person.age);
    println!("Person : {:?}", person);
    */

    let p1 = init_point();
    let p2 = Box::new(from_point(4.0,5.0));
    println!("Origin on Stack : [{},{},{}]", p1.x, p1.y, p1.z);
    println!("Origin on Heap: [{},{},{}]", p2.x, p2.y, p2.z);

    println!("Sizeof Point on Stack : {}", mem::size_of_val(&p1)); // whole struct is in memeory
    println!("Sizeof Point on Heap: {}", mem::size_of_val(&p2)); // size of pointer

    //Assignmet from struct on heap
    let p3 = *p2;
    println!("p3 assigned from p2: [{},{},{}]", p3.x, p3.y, p3.z);

    // init explicitly
    let p4 = Point{x:10.0,y:20.0,z:30.0};
    println!("Init p4 with 10,20,30: [{},{},{}]", p4.x, p4.y, p4.z);

    // udate syntaxes, copy everything except a few from other instance
    // init explictly z but copy other members from other
    let p5 = Point{z:100.0,..p4};
    println!("Using update syntax, should init x,y from p4: [{},{},{}]", p5.x, p5.y, p5.z);

    //struct using other struct
    let line = Line{start:p3,end:p4};
    println!("Line -> [{},{}], [{},{}]", line.start.x,line.start.y,line.end.x,line.end.y);

    println!("{:?}",p5);
    println!("{:?}",line);    
}

fn struct_funcs_and_convertion_func_demo() {
    
    //implement a member function, can have multiple impl block
    let p6 = Point{x:100.0,y:200.0,z:300.0};
    p6.show();

    println!("Demoing Conversion from float to Point");
    let p7 = Point::from(30.0);
    p7.show();

    let flt:f64 = p7.into();
    println!("sym point is : [{},{},{}]",flt,flt,flt );
}


fn tuple_struct_demo() {
    struct Position(i32,i32,i32);
    let ps = Position(10,100,1000);
    println!("tuple-struct Pos : [{0},{1},{2}]", ps.0,ps.1,ps.2);

    #[derive(Debug)]
    struct Color(i32,i32,i32);
    let color = Color(255,0,0);
    // color = ps; Even though two tuple struct has same type, can not assign to each other
    println!("Color : [{0},{1},{2}]", color.0,color.1,color.2);
    println!("{:?}",color);    
}

pub fn rust_struct_demo() {

    //init, access, assign 
    init_struct_demo();

    //tuple-struct : create struct without named fields
    tuple_struct_demo();

    //implemnt member func and from to make it conversion compliant
    struct_funcs_and_convertion_func_demo();   

    
}
