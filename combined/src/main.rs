#[allow(unused)]

use std::io::{self, Write};

mod combined;
mod more_combo;

#[derive(Debug)]
enum Errors
{
    Error1,
    Error2,
}

impl std::fmt::Display for Errors
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        match self
        {
            Errors::Error1 => write!(f, "Eror_1"),
            Errors::Error2 => write!(f, "Eror_2"),
        }
    }
}

fn main() {
    let error = Errors::Error1;
    println!("{}", error);
    println!("{:?}", error);
    //combined::run();
    //more_combo::run();
}
