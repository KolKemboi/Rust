use std::{fmt::write, io::{self, Write}};
#[warn(unused_imports)]

enum InputError
{
    EmptyInput,
    InvalidAge,
}

impl std::fmt::Display for InputError
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        match self 
        {
            InputError::EmptyInput => write!(f, "Input cannot be Empty"),
            InputError::InvalidAge => write!(f, "Please enter a valid age, an integer"),
        }
    }
}

struct User
{
    name: String,
    age: u8,
}

fn get_age_input() -> Result<u8, InputError>
{
    let mut input = String::new();
    io::stdout().flush().unwrap(); //Ensures prompt appears first
    io::stdin().read_line(&mut input).expect("Failed to readline");

    let input = input.trim();

    if input.is_empty()
    {
        return Err(InputError::EmptyInput);
    }
    match input.parse::<u8>()
    {
        Ok(age) => Ok(age),
        Err(_) =>Err(InputError::InvalidAge),
    }
    
}

fn get_name_input<'a>() -> Result<String, InputError>
{
    let mut input = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("Failed to readline");

    let name = input.trim();
    if name.is_empty()
    {
        Err(InputError::EmptyInput)
    }
    else
    {
        Ok(name.to_string())    
    }


}

pub fn run()
{
    loop 
    {
        println!("Enter Name:");
        let name = match get_name_input()
        {
            Ok(n) => n,
            Err(e) =>
            {
                println!("Error, {}", e);
                continue;
            }
        };

        println!("Enter Age:");
        let age = match get_age_input()
        {
            Ok(a) => a,
            Err(e) => {
                println!("Error {}", e);
                continue;
            }
        };

        let user = User{name, age};

        print!("User created, {}, {}", user.name, user.age);
        break;

    }
}