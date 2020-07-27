# Struct
Similar to other OOP languages structs grous related data and provide methods to access/modify these data
```rust
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
```
_An `impl` section in struct can have more than one function and a struct can have multiple `impl` section._

This is how you instantiate struct and access its data directly
```rust
    let p1 = init_point();
    let p2 = Box::new(from_point(4.0,5.0));
    println!("Point on Stack : [{},{},{}]", p1.x, p1.y, p1.z);
    println!("Point on Heap: [{},{},{}]", p2.x, p2.y, p2.z);
```
## Update Syntax
Rust provoides a way to init ojbect partially and remaining using other instances. Below init everything except z using p4
```rust
    let p5 = Point{z:100.0,..p4};
    println!("Using update syntax, should init x,y from p4: [{},{},{}]", p5.x, p5.y, p5.z);
```
## Tuple Structs
RUST provide ad way to create a struct without having any named field, still two structs with same datyped member have unique identidy. One can access its data memebers using its index i.e. 0,1,2..
```rust
    struct Position(i32,i32,i32);
    let ps = Position(10,100,1000);
    println!("tuple-struct Pos : [{0},{1},{2}]", ps.0,ps.1,ps.2);

    struct Color(i32,i32,i32);
    let color = Color(255,0,0);    
```
Below line will not compile as ps and color are two different struct 
```rust
    color = ps;
```    

## Enum Types
Enum defines a type enumerating its possible variants. In RUST enums are not restricted to only integral types.
```rust
    enum Color
    {
        Red,
        Green,
        Blue,
        RGBColor(u8,u8,u8), //tuple
        CMYKColor{c:u8,m:u8,y:u8,b:u8}, //struct
        Custom(String),
    }
```
Even enums can have function as structs
```rust
    impl Color {
        fn show_name(&self,name:String) {
            println!("Cusotm Color : {}", name);
        }
    }
```
Here is how pattern matching is done with various enum variants
```rust
    //let color:Color = Color::RGBColor(10,23,45);
    //let color:Color = Color::RGBColor(0,0,0);    
    //let color:Color = Color::CMYKColor{c:10,m:23,y:45,b:255};

    let color = Color::Name(String::from("special red"));
    
    match color 
    {
        Color::Red => println!("You chose Red"),
        Color::Green => println!("You chose Green"),
        Color::Blue => println!("You chose Blue"),
        Color::RGBColor(0,0,0)=>println!("You chose Black"),
        Color::CMYKColor{c:_,m:_,y:_,b:255,} => println!("You chose Black"),
        Color::RGBColor(r,g,b) =>println!("You chose RGB:[{},{},{}]",r,g,b), 
        Color::Name(s) =>println!("You go by given color :{}",s),
        _=> println!("In Valid Option")
    }
```
As we know _ means 'for any other'

## Matching with Option<T>
RUST provides two very useful enums 
* Some : when result is possible
* None : when result is not possible 
let us go by our classic division by zero example
```rust
    fn division_by_zero(nr:f64, dr:f64) {
        //let res:Option<f64> = Some(nr/dr);
        let res = if dr != 0.0 { Some(nr/dr)} else {None};
        match res {
            Some(z) => println!("{}/{} = {}",nr,dr,z),
            None => println!("Division not possible if dr is zero")
        }
    }
```
Other Some/None examples : arrays, 
### If let / while let
When we are interested in one of all matching options,we use `if let` to avoid more typing, indentation etc
```rust
    let ans = Some(nr/dr);
    match ans {
        Some(3) => println!("three"),
        _ => println!("not important"),
    }
```
Now we use `if let`
```rust
    if let Some(3) = ans {
        println!("three");
    } else {
        println!("not important");
    }
```
Similar one for `while let`
```rust
    let mut a = vec::new();
    a.push(1);
    a.push(2);
    a.push(3);
    while let Some(x) = a.pop() {
        println!(x);
    }
```
## Conversion 
Uses `from` and `into` traits
### From
Create itself from other type
```rust 
let ref_str = "hello";
let ly_string = String::from(ref_str);
```
e.g. convert float value to Point
```rust
impl From<f64> for Point {
    fn from (val:f64) ->Self {
        Point{x:val, y:val, z:val}

    }
}
```
### Into
reverse of from, convert itself to another type.
If `from` is implemented `into` will call it 
## String
There are two type of strings String, vec of char, created on Heap and not null terminated &str is slice used to view String