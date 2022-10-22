use std::env;
use std::fs;
use glob::glob;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Error: Please enter exactly one directory path");
        return;
    } 

    let dir_path = &args[1];
    let search_path = dir_path.clone() + "/*";

    let mut line_count = 0;
    for file in glob(&search_path).expect("Failed to read glob pattern"){
        let read_file = fs::read_to_string(file.unwrap()).unwrap();
        let count = read_file.lines().count();
        line_count += count;
    }
    println!("The directory {} have {} lines of text in it.", &dir_path, line_count);
}
