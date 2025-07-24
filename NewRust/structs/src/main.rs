use std::collections::HashMap;

fn main() {
    struct UsrData {
        name: String,
        age: u32,
    }

    struct OtherUsr {
        pos: String,
        user_info: HashMap<String, String>,
    }

    let usr_new: UsrData = UsrData {
        name: (String::from("Kolll")),
        age: (10),
    };

    println!("Hello, world!");
    println!("{}, {}", usr_new.name, usr_new.age);

    let mut usr_info: HashMap<String, String> = HashMap::new();
    usr_info.insert(String::from("Name"), String::from("Loll"));

    let other_usr: OtherUsr = OtherUsr {
        pos: String::from("Bos"),
        user_info: usr_info,
    };
}
