#[allow(unused)]
use std::io::{self, Write};

mod gen_life_traits;

trait Summary {
    fn summarize(&self) -> String;
}

pub struct Article
{
    pub headline: String,
    pub author: String,
    pub content: String,
}

impl Summary for Article
{
    fn summarize(&self) -> String {
        return format!("{}, {}", self.headline, self.author);
    }
}
fn main() {
    let data = vec![1, 2, 3, 4, 5];
    let largest = gen_life_traits::largest(&data);
    io::stdout().flush().unwrap();
    println!("in {:?}, this {} is the largest", data, largest);

    let article = Article{
        headline: String::from("Kol"), 
        author: String::from("value"), 
        content: String::from("Jdjklfjds")};
    

    let string1 = "tust";
    let string2 = "ust";

    let res = longest_with_summary(&string1, &string2, article);
    println!("{}", res);
}



fn longest_with_summary<'a, 'b, T>(x: &'a str, y: &'a str, ann: T) ->&'b str
where 
    T: Summary,
    'a: 'b,
    {
        println!("Announcement: {}", ann.summarize());
        if x.len() > y.len()
        {
            return x;
        }
        else {
            return y;
        }
    }