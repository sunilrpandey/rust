# Complex Types and More

- [ ] [Struct](#struct)
- [ ] [Update Syntax](#updatesyntax)
- [ ] [Tuple Struct](#tuplestruct)
- [ ] [Enums](#enums)
- [ ] [Matching with Option](#match)
- [ ] [If let / while let](#iflet)
- [ ] [Conversion/Casting](#conversion)
    - [Type Conversion using from/into](#frominto)
- [ ] [String](#string)
- [ ] [Misc](#misc)


# <a name=struct>Struct</a>
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
## <a name=updatesyntax>Update Syntax</a>
Rust provoides a way to init ojbect partially and remaining using other instances. Below init everything except z using p4
```rust
    let p5 = Point{z:100.0,..p4};
    println!("Using update syntax, should init x,y from p4: [{},{},{}]", p5.x, p5.y, p5.z);
```
## <a name=tuplestruct>Tuple Structs</a>
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

## <a name=enums>Enum Types</a>
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

## <a name=match>Matching with Option<T></a>
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
Other Some/None examples : arrays, stack, pointers
## <a name=iflet> If let / while let
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

## <a name=string>String</a>
There are two type of strings String, vec of char, created on Heap and not null terminated &str is slice used to view String

## <a name=conversion>Conversion/Casting</a> 

No implicit conversion in RUST, use `as` for explicit conversion wherever allowed. e.g A float cannot be directly converted to a char, if required convert it to int then char 
```rust
let fl_num = 66.4341_f32;
let int_num = fl_num as u8;
let char_val = int_num as char;
println!("Casting: {} -> {} -> {}", fl_num, int_num, char_val);
```
### Conversion provided by Libs
numeral to String
```
let i = 5;
let five = String::from("5");

assert_eq!(five, i.to_string());
```
### <a name=/frominto>Make Type convertible using From/Into traits</a>
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


## <a name=mis>MISC</a>
### Type is used for aliasing, similar to `using` in C++11 
```rust 
type Operations = VeryVerboseEnumOfThingsToDoWithNumbers; 
type Price = u64;
```

### Static keyword
static: A possibly mutable variable with 'static lifetime. The static lifetime is inferred and does not have to be specified. Accessing or modifying a mutable static variable is unsafe.
```rust
static LANGUAGE: &str = "Rust";
```

