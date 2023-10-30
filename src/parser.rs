use std::fs;

pub fn get_input() {
    let contents = fs::read_to_string("test.txt")
        .expect("Should have been able to read the file");
    println!("{contents}");
}