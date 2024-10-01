use std::fs::{self, OpenOptions};
use std::io::{self, Read, Write};

struct Car {
    year: u32,
    make: String,
    model: String,
}

fn reading_from_console() {
    let mut buffer = String::new();

    print!("What year is your car? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let year: u32 = buffer.trim().parse().unwrap();
    buffer.clear();

    print!("What's the make? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let make = buffer.trim().to_string();
    buffer.clear();

    print!("What's the model? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let model = buffer.trim().to_string();

    let file_path = "user_info.txt";
    let content = format!("{} {} {}", year, make, model);
    fs::write(file_path, content).unwrap();

    println!("Information saved to file.");

    let contents = fs::read_to_string(file_path).unwrap();
    println!("Content of user_info.txt:\n{}", contents);
}

fn main() {
    reading_from_console();

    
}