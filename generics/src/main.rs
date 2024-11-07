#[allow(unused)]

use std::collections::HashMap;
use std::hash::Hash;

fn main()
{
    let data = Data::new("info", "endpoint");
    data.ret_endpoint();
    data.ret_info();

    let mut stack = Stack::new();
    println!("{}", stack.is_empty());

    stack.push(10);
    stack.push(1);
    stack.push(0);
    stack.push(110);
    
    println!("{:?}", stack.items);

    match stack.pop() {
        Some(val) =>println!("{}", val),
        None => println!("Is empty"),
    }

    match stack.peek()
    {
        Some(val) => println!("{}", val),
        None => println!("Nothing to peak"),
    }

    let mut hashi = Container::<String, i32>::new();
    hashi.add_value(String::from("v_1"), 10);
    hashi.add_value(String::from("v_2"), 11);
    hashi.add_value(String::from("v_3"), 12);
    hashi.add_value(String::from("v_4"), 13);

    let key = String::from("v_2");
    match hashi.get_value(&key)
    {
        Some(value) =>println!("{}", value),
        None => println!("No Key or value"),
    }

}

struct Container<K, V>
where
    K: Eq + Hash
{
    items: HashMap<K, V>,
}

impl<K, V> Container<K, V>
where
    K: Eq + Hash
{
    fn new() -> Self {
        Container {
            items: HashMap::new(),
        }
    }

    fn add_value(&mut self, key: K, value: V) {
        self.items.insert(key, value);
    }

    fn get_value(&mut self, key: &K) ->Option<&V>
    {
        self.items.get(key)
    }

}


struct Stack<T>
{
    items:Vec<T>,
}

impl<T> Stack<T>
{
    fn new() ->Self
    {
        Stack { items: Vec::new() }
    }

    fn push(&mut self, item: T)
    {
        self.items.push(item);
    }

    fn pop(&mut self) -> Option<T>
    {
        self.items.pop()
    }

    fn peek(&mut self) ->Option<&T>
    {
        self.items.last()
    }

    fn is_empty(&mut self) ->bool
    {
        self.items.is_empty()
    }

}

#[warn(dead_code)]
trait InfoShow
{
    fn showinfo(&self) ->String;
}
struct Data<T, U>
{
    info: T,
    endpoint: U,
}

impl <T: std::fmt::Display, U: std::fmt::Display> Data<T, U>
{
    fn new(info: T, endpoint: U) ->Self
    {
        Data { info: info, endpoint: endpoint }
    }    
    fn ret_info(&self)
    {
        println!("{}", self.info);
    }
    fn ret_endpoint(&self)
    {
        println!("{}", self.endpoint);
    }
}

impl<T: std::fmt::Display, U: std::fmt::Display> InfoShow for Data<T, U>
{
    fn showinfo(&self) -> String {
        format!("Info: {}, Endpoint: {}", self.info, self.endpoint)
    }
}