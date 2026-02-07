use std::fs;
use std::env;

fn main() {
    /*
        grep search_term file

    */
    let args: Vec<String> = env::args().collect();
    let command = &args[1];
    let search_term = &args[2];
    let file_location = &args[3];
    let contents = fs::read_to_string(file_location).expect("File read error");
    print!("{}:{}", command, search_term);
    println!("{contents}\n");
}
