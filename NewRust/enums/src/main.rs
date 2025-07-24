enum Direction {
    North,
    South,
}

fn main() {
    let dir: Direction = Direction::North;
    match dir {
        Direction::North => println!("This is north"),
        Direction::South => println!("This is south"),
    }
}
