use std::collections::HashMap;

fn main() {
    let int: i32 = 20;
    let int_2: i32 = int + 10;

    println!("{}, {}", int, int_2);
    println!("{} ", int + int_2);

    let bool_1: bool = true;
    let bool_2: bool = !bool_1;
    println!("{}", bool_1);
    println!("{}", bool_2);

    let float_1: f32 = 10.0;
    let float_2: f64 = 10.0;
    println!("{}, {}", float_1, float_2);
    println!("{}, ", float_2);

    let char_1: char = 'R';
    let char_2: char = '2';
    let char_3: char = 'D';
    println!("{}{}{}{}", char_1, char_2, char_3, char_2);
    // This is to the heap
    let mut str_1: String = String::from("Hello World");
    println!("{}", str_1);
    str_1.push('!');
    println!("{}", str_1);
    // This is in the heap
    // you can use the :char types to append to the string
    let mut str_2: String = String::new();
    str_2.push(char_1);
    str_2.push(char_2);
    str_2.push(char_3);
    str_2.push(char_2);
    println!("{}", str_2);
    str_2.pop(); // this removes the last value
    println!("{}", str_2);
    str_2.remove(1); // this removes a certain index
    println!("{}", str_2);
    // This is in the stack
    let str_3: &str = "Immutable String";
    println!("{}", str_3);

    //BASIC DSA
    //lists
    let list: [i32; 3] = [1, 2, 3];
    println!("{}", list[1]);

    //HASHMAPS
    let mut mapu: HashMap<&str, i32> = HashMap::new();
    mapu.insert("Hello", 1);
    mapu.insert("World", 2);

    for (key, value) in &mapu {
        println!("{}, {}", key, value);
    }
    //TUPLE
    let tuple: (&str, i32) = ("Kol", 10);
    println!("{}, {}", tuple.0, tuple.1);
}
