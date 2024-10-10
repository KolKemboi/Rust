#[allow(unused)]

use std::collections::HashMap;

pub fn vecs()
{
    let mut v = vec![1, 2, 3, 4];
    v.push(10);
    println!("{}", &v[4]);
}

pub fn hashimapu()
{
    let mut score = HashMap::new();
    score.insert(String::from("v_1"), 100);
    score.insert(String::from("v_2"), 200);
    score.insert(String::from("v_3"), 300);
    score.insert(String::from("v_4"), 400);

    let mut get_score = String::new();
    get_score.push_str("v_1");
    let out = score.get(&get_score);

    match out
    {
        Some(retd) => println!("{}", retd),
        None => println!("no value found"),
    }

    for (key, val) in &score
    {
        println!("{}, {}", key, val);
    }
    
}