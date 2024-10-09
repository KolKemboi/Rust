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

}

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
