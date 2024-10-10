#[allow(unused)]
#[warn(dead_code)]

use std::collections::HashMap;

mod collections;
mod libs;

fn main() {
    println!("Hello, world!");

    collections::vecs();
    collections::hashimapu();
    libs::enum_hashmaps::run();
}

