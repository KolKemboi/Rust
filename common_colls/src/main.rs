#[allow(unused)]
#[warn(dead_code)]

use std::collections::HashMap;

mod collections;
mod libs;

fn main() {
    println!("Hello, world!");

    collections::vecs();
    collections::hashimapu();
    libs::enum_hashmaps::run();
}

enum Dat
{
    Num_1    Num_2,
}
impl Dat {
    fn run(&self)
    {
        println!("{}", String::from("value");)
    }
}

fn run()
{
    let dat = Dat{
        Num_1: 20,
        Num_2: 30,
    };

}