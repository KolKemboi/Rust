#[allow(unused)]


fn other() {
    
    println!("Hello, world!");

    let pair = Pair::new(10, "Hello");
    println!("{}", pair.get_first());
    println!("{}", pair.get_second());
    println!("{}", pair.describe());
}

trait Describe {
    fn describe(&self) ->String;
}

struct Pair<T, U>
{
    first: T,
    second: U,
}

impl <T, U> Pair<T, U>
{
    fn new(first: T, second: U) ->Self
    {
        Pair { first: first, second: second }
    }
    fn get_first (&self) -> &T 
    {
        &self.first
    }
    fn get_second (&self) -> &U 
    {
        &self.second
    }
    
}

impl<T: std::fmt::Display, U: std::fmt::Display> Describe for Pair<T, U>
{
    fn describe(&self) ->String {
        format!("pair contains {}, {}", self.first, self.second)
    }
}