#![allow(dead_code)]
enum Color
{
    Red,
    Green,
    Blue,
    RGBColor(u8,u8,u8), //tuple
    CMYKColor{c:u8,m:u8,y:u8,b:u8}, //struct
    Name(String),
}
impl Color {
    fn show_name(&self,name:String) {
        println!("Color type to set : {}", name);
    }
}
pub fn rust_enum_test() {

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

    println!("Enum function Demo");
    let c = Color::Name(String::from("special red"));
    c.show_name(String::from("Custom"));
}