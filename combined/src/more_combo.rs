use std::io::{self, Write};

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
            InputError::EmptyInput => write!(f, "Empty Input"),
            InputError::InvalidAge => write!(f, "Invallid AGe"),
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
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("readling Eror");

    let input = input.trim();

    if input.is_empty()
    {
        return  Err(InputError::EmptyInput);
    }
    match input.parse::<u8>()
    {
        Ok(age) => Ok(age),
        Err(_) => Err(InputError::InvalidAge),
    }
}

fn get_name_input() -> Result<String, InputError>
{
    let mut name = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut name).expect("readline error");

    let name = name.trim();
    
    if name.is_empty()
    {
        return Err(InputError::EmptyInput);
    }
    else 
    {
        return Ok(name.to_string());    
    }

}

pub fn run()
{
    loop 
    {
        println!("Enter Name");
        let name = match get_name_input()
        {
            Ok(n) => n,
            Err(e) =>
            {
                println!("Error, {}", e);
                continue;
            }
        };

        println!("Get Age");
        let age = match get_age_input()
        {
            Ok(a) => a,
            Err(e)=>
            {
                println!("Error, {}", e);
                continue;
            }
        };

        let user = User{name, age};

        println!("User {}, {}", user.name, user.age);
        break;
    }
}