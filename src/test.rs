use std::fs;

pub fn main() {
    let content = fs::read_to_string("main.c").expect("Could not read main.c");
    println!("{}", content);
}
