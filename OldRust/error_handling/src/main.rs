#[allow(unused)]

use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    refresher();
    enums_structs();

    
}

fn refresher()
{
    let mut hashimapu = HashMap::new();
    hashimapu.insert(String::from("v_1"), "100");
    hashimapu.insert(String::from("v_2"), "400");
    hashimapu.insert(String::from("v_3"), "300");

    let key = String::from("v_1");

    match hashimapu.get(&key) 
    {
        Some(value) => println!("{}", value),
        None => println!("Failed"),
    }

    let vector = vec![1, 2, 3, 4, 5];
    for val in &vector
    {
        println!("{}", val);
    }
    println!("{}", vector.len());

}

fn enums_structs()
{
    struct Point
    {
        x: i32,
        y: i32,
    }

    enum Shapes
    {
        Triangle(Point, Point, Point),
    }

    impl Shapes
    {
        fn out_shape(&self)
        {
            println!("Self");
        }    
    }

    let p1 = Point{
        x: 12,
        y: 12,
    };
    let p2 = Point{
        x: 12,
        y: 12,
    };
    let p3 = Point{
        x: 12,
        y: 12,
    };
    

    let new_tri = Shapes::Triangle(p1, p2, p3);

    match &new_tri
    {
        Shapes::Triangle(p1, p2, p3) => println!("{}, {}, {}", p1.x, p2.y, p3.x),
    }

    new_tri.out_shape();
}