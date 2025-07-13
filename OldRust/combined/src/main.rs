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
   // println!("{}", error);
    println!("{:?}", error);
    //combined::run();
    //more_combo::run();
    let new_event = Event::BACKWARD;
    new_event.event();
}



enum Event
{
    FORWARD,
    BACKWARD,
    RIGHT,
    LEFT,
}

impl Event
{
    fn event(&self)
    {
        match self
        {
            Event::BACKWARD => println!("Went Back"),
            Event::FORWARD => println!("Went Forward"),
            Event::RIGHT => println!("Went rigth"),
            Event::LEFT => println!("Went Left"),
        }
    }
}