use std::fs;
use crate::graph::{self, ArgumentationFramework};
pub fn get_input() {
    let contents = fs::read_to_string("test.txt")
        .expect("Should have been able to read the file");
    let a = contents.trim().split('\n');
    let b : Vec<&str> = a.collect();
    let mut info = b[0].split_ascii_whitespace();
    let p = info.next().unwrap();
    let af = info.next().unwrap();
    let n = info.next().unwrap().parse::<usize>().unwrap();
    println!("{}", n);
    let af = ArgumentationFramework::new(n);
}