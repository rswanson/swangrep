use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1];
    let path = &args[2];

    // println!("In file: {}", path);
    let contents = fs::read_to_string(path)
        .expect("Should have been able to read the file but could not");

    // println!("With text: {}",contents);

}
