trait Shape {
    fn create(in_name:&'static str) -> Self;
    fn name(&self)->&'static str;
    fn render(&self) {
        println!("{} abstract shape, can't render",self.name());
    } 
}
impl Shape for Square {
    fn create(in_name:&'static str) -> Square
    {
        Square{name:in_name}
    }
    fn name(&self)->&'static str {
        self.name
    }
    fn render(&self) {
        println!("{} rendered.",self.name());
    }
}
struct Square
{
    name:&'static str
}

// Circle created with trait Shape
impl Shape for Circle 
{
    fn create(in_name:&'static str) -> Circle
    {
        Circle{name:in_name}
    }
    fn name(&self)->&'static str {
        self.name
    }
    fn render(&self) {
        println!("{} rendered.",self.name());
    }
}
struct Circle
{
    name:&'static str
}

trait Summable<T>
{
    fn sum(&self) -> T;
}
impl Summable<i32> for Vec<i32>
{
    fn sum(&self) -> i32 {
        let mut result = 0;
        for i in self {
            result += *i;
        }
        return result;
    }
}

#[derive(Clone)] // we add the Clone trait to MyStruct
struct MyStruct {
   mem_f: f32,
   mem_i: i64,
}

fn use_lib_traits() {
   let st = MyStruct { mem_f: 5.0, mem_i: 6 };
   //let copy = st;
   //let copy = &st;
   let copy = st.clone(); // clone it!
   println!("copy : [{},{}]",copy.mem_f,copy.mem_i);
   println!("st : [{},{}]",st.mem_f,st.mem_i);
}

pub fn traits_test() {
    
/*     //let shape:Square = Square{name:"My Square"};
    let shape:Square = Shape::create("My Square");
    shape.render();
    
    let shape = Circle{name:"My Circle"};
    shape.render();

    let v = vec![30,20,10];
    println!("Sum of elements : {}", v.sum());
 */
    use_lib_traits();
}
