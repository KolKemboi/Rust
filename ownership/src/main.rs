#[allow(unused)]


fn main() {
    println!("Hello, world!");
    basic();
    move_semantics();
    cloning();
    borow_ref();
    mutable_ref();
}

fn mutable_ref()
{
    let mut s = String::from("value");
    change(&mut s);
    println!("{}", s);
}

fn change(s: &mut String)
{
    s.push_str("string");
}

fn borow_ref()
{
    let s1 = String::from("Length");
    let len = calc_len(&s1);
    println!("The string {} is {} long", s1, len);
}

fn calc_len(s: &String) ->usize
{
    return s.len();
}


fn cloning()
{
    let s1 = String::from("To be cloned");
    let s2 = s1.clone();
    println!("{}", s1);
    println!("{}", s2);
}


fn move_semantics()
{
    let s1 = String::from("Hello");
    let s2 = s1;
    println!("{}", s2);
}


fn basic()
{
    let s = String::from("Hello World");
    println!("{}", s);
}