use std::collections::HashMap;

pub fn out()
{
    println!("hello world");
}

pub fn enums()
{
    enum Data
    {
        Offline,
        Online, 
    }

    let on_data = Data::Online;
    let off_data = Data::Offline;
    match on_data {
        Data::Offline =>println!("{}", String::from("Offline")),
        Data::Online =>println!("{}", String::from("Online")),
    }
    if let Data::Offline = off_data
    {
        println!("Offline");
    }
}

pub fn hashimapus()
{
    let mut hasi = HashMap::new();
    let mut value_1 = vec![1, 2, 3];
    let mut value_2 = vec![2, 3, 4];
    let mut value_3 = vec![4, 5, 6];
    hasi.insert(String::from("v_1"), value_1);
    hasi.insert(String::from("v_2"), value_2);
    hasi.insert(String::from("v_3"), value_3);

    let key = String::from("v_2");

    match hasi.get(&key) {
        Some(value) => println!("{:?}", value),
        None => println!("None"),
    }

}