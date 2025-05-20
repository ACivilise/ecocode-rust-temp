use std::path::PathBuf;

fn main() {
    // EC2 rule will detect this nested if-else pattern
    let number = 3;
    
    if number == 2 {    
        println!("Number is 2");
    } else if number == 3 {
        println!("Number is 3");
    } else if number == 4 {
        println!("Number is 4");
    }
    
    // A more eco-friendly approach using match
    match number {
        2 => println!("Number is 2 - using match"),
        3 => println!("Number is 3 - using match"),
        4 => println!("Number is 4 - using match"),
        _ => println!("Number is something else"),
    }
    
    // Another example that will trigger the warning
    let path = PathBuf::from("..");
    if path.exists() {
        println!("Path exists");
    } else if path.is_dir() {
        println!("Path is a directory");
    } else if path.is_file() {
        println!("Path is a file");
    }
}