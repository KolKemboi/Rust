#![allow(unused)]

use std::{io}; //this is to get info from the user

fn main() {
    println!("Hello, world!");
}

//Using structs and enums
struct Task{
    desc: String,
    comp: bool,
}

enum Command {
    Add(String),
    View,
    Completed(usize),
    Remove(usize),
}

fn get_user_input() -> String
{
    let mut usr_input = String::new();//heap allocated
    io::stdin().read_line(&mut usr_input).expect("Failed to get usr input");//usr input with error handling
    return  usr_input.trim().to_string();
}

fn add_task(tasks: &mut Vec<Task>, desc: String)
{
    tasks.push(Task{desc, comp: false});
}

fn view_task(tasks: &mut Vec<Task>)
{
    if tasks.is_empty()
    {
        println!("No Tasks Found");
    }
    else 
    {
        for (index, task) in tasks.iter().enumerate()
        {
            let status =  if task.comp {"Done"} else {"Not Done"};
            println!("{}: [{}] {}", index, status, task.desc);
        }
    }
}

fn completed_task(tasks: &mut Vec<Task>, index: usize) -> Result<(), String>
{
    if index < tasks.len() 
    {
        tasks[index].comp = true;
        println!("Task {} marked as completed", index);
        Ok(())
    }
    else
    {
        Err(format!("Task {} does not exist", index))
    }
}
