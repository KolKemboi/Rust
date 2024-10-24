use std::collections::HashMap;

pub fn run()
{
    println!("Hello World");
}

pub fn structs()
{
    struct Rectangle 
    {
        width: i32,
        height: i32,
    }

    impl Rectangle
    {
        fn get_area(&self) -> i32
        {
            let area = self.width * self.height;
            return  area;
        }    
    }

    let new_rect = Rectangle
    {
        width: 30,
        height: 30,
    };
    
    let rect_area = new_rect.get_area();
    println!("{}", rect_area);


}

pub fn enums()
{
    struct Point
    {
        x: i32,
        y: i32,
    }

    enum NewRectangle
    {
        Width(Point),
        Height(Point),
    }

    impl NewRectangle 
    {
        fn get_points(&self)
        {
            match self
            {
                NewRectangle::Width(point) => {
                    println!("{}, {}", point.x, point.y);
                }    
                NewRectangle::Height(point) => {
                    println!("{}, {}", point.x, point.y);
                }
            }
        }    
    }

    let width_pts = Point
    {
        x: 10,
        y: 30,
    };
    let height_pts = Point
    {
        x: 100,
        y: 40,
    };

    let rect_width = NewRectangle::Width(width_pts);
    let rect_height = NewRectangle::Height(height_pts);

    rect_width.get_points();
    rect_height.get_points();
}

pub fn hashimapu()
{
    let mut hashi = HashMap::new();

    let v_1 = vec![1, 2, 3];
    let v_2 = vec![2, 2, 3];
    let v_3 = vec![3, 2, 3];

    hashi.insert(String::from("v_1"), v_1);
    hashi.insert(String::from("v_2"), v_2);
    hashi.insert(String::from("v_3"), v_3);

    let key = String::from("v_1");

    match hashi.get(&key)
    {
        Some(value) => for elem in value.iter() {
            println!("{}", elem);
        } ,
        None => println!("No value"),
    }


}