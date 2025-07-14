use std::collections::HashMap;

fn main() {
    let mut mapu: HashMap<i32, &str> = HashMap::new();
    mapu.insert(0, "Hello");
    mapu.insert(1, "World");
    mapu.insert(3, "!");

    for (key, value) in &mapu {
        println!("{}, {}", key, value);
    }

    let mut num = 10;
    if num > 10 {
        println!("Bigger Than 10");
    } else if num < 10 {
        println!("Smaller Than 10");
    } else {
        println!("The number is 10");
    }

    loop {
        println!("the number is {}", num);
        num = num - 1;

        if num < 0 {
            break;
        }
    }

    while num < 10 {
        println!("The num is {}", num);
        num = num + 1;
        if num > 10 {
            break;
        }
    }

    for i in 0..num {
        println!("The num is {}, {}", i, num);
    }
    for i in 0..=num {
        println!("The num is {}, {}", i, num);
    }

    let list = ["Hello", "World", "!"];
    for thing in list.iter() {
        println!("{}", thing);
    }
}
