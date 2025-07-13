#[allow(unused)]
use std::fmt;

fn main() {
    println!("Hello, world!");

    let black = Color(0, 0, 0);
    let red = Color(255, 0, 0);

    println!("{}, {}, {}", black.0, black.1, black.2);
    println!("{}, {}, {}", red.0, red.1, red.2);

    let user = User
    {
        name: String::from("value"),
        age: 32,
        email: String::from("@gmail"),
    };
    user.out_user();

    let home_ip = IpAddr::V4(String::from("127.0.0.1"));
    let loop_ip = IpAddr::V6(String::from("127.0."));
    
    let num: u8 = 1;
    check_number(num);
    let new_shape = Shape::Rectangle { width: 10.0, height: 20.0 };
    new_shape.show_shape();
    
}

struct Point
{
    x: i32,
    y: i32,
}

enum Shape
{
    Circle{radius: f64},
    Rectangle {width: f64, height: f64},
    Trianlgw(Point, Point, Point),
}

impl Shape
{
    fn show_shape(&self)
    {
        let shape = Shape::Rectangle { width: 10.0, height: 40.0 };
        let point1 = Point{x: 6, y:7};
        let point2 = Point{x: 6, y:7};
        let point3 = Point{x: 6, y:7};
        let shape2 = Shape::Trianlgw(point1, point2,  point3);

        match shape
        {
            Shape::Circle { radius } =>println!("circle with rad {}", radius),
            Shape::Rectangle { width, height } =>println!("Rectang with {}, {}", width, height),
            Shape::Trianlgw(p1, p2, p3) => {
                println!("{}, {}, {}, {}, {}, {}", p1.x, p1.y, p2.x, p2.y, p3.x, p3.y);
            }
        }
    }
    
}


fn check_number(num: u8)
{
    match num
    {
     1 | 2 | 3 => println!("Small"),
     4..=6 => println!("Mid"),
     _ => println!("large"), 
    }
}


#[derive(Debug)]
enum IpAddr
{
    V4(String),
    V6(String),
}



struct User
{
    name: String,
    age: i32,
    email: String,
}
impl User 
{
    fn out_user(&self)
    {
        println!("{}, {}, {}", self.name, self.age, self.email);
    }    
}


//Basic Structs
struct Color(i32, i32, i32);
