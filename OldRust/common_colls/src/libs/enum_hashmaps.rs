use std::collections::HashMap;

pub fn run()
{
    enum DevStat
    {
        Online,
        Offline,
        Error(String),
    }

    struct Dev
    {
        id: i32, 
        name: String,
    }

    let mut dev_stats: HashMap<i32, DevStat> = HashMap::new();

    dev_stats.insert(1, DevStat::Online);
    dev_stats.insert(2, DevStat::Offline);
    dev_stats.insert(3, DevStat::Error(String::from("Error in Device")));

    let devices = vec![
        Dev{id: 1, name: String::from("Device A")},
        Dev{id: 2, name: String::from("Device B")},
        Dev{id: 3, name: String::from("Device C")},
    ];

    for device in devices
    {
        match dev_stats.get(&device.id)
        {
            Some(DevStat::Online) => println!("{} is online", device.name),    
            Some(DevStat::Offline) => println!("{} is offline", device.name),    
            Some(DevStat::Error(err)) => println!("{} has an error {}", device.name, err),
            None => println!("{} has no status recorded", device.name),    
        }
    }



}