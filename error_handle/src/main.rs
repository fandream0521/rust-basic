#![allow(unused)]
use std::{
    fs::{self, File},
    io::Read,
};
fn main() {
    let content = r#"
    Hello
    World"#;
    println!(
        "Last char of first line: {:?}",
        last_char_of_first_line(content)
    );
    println!("content: {:#?}", content);
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

fn read_content_from_file() {
    let path = "test.txt";
    let content = read_username_from_file(path);
    match content {
        Ok(content) => {
            println!("Content: {}", content);
        }
        Err(error) => {
            panic!("Error reading file: {:?}", error);
        }
    }
    fs::read_to_string(path).unwrap();
}
#[derive(Debug)]
struct OurError;

impl From<std::io::Error> for OurError {
    fn from(_: std::io::Error) -> Self {
        println!("Error reading file");
        OurError
    }
}

fn read_username_from_file(path: &str) -> Result<String, OurError> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn open_file() {
    let path = "test.txt";
    let file = File::open(path);
    match file {
        Ok(_) => {
            println!("File opened successfully");
        }
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound => match File::create(path) {
                Ok(_) => {
                    println!("File created successfully");
                }
                Err(error) => {
                    panic!("Error creating file: {:?}", error);
                }
            },
            _ => {
                panic!("Error opening file: {:?}", error);
            }
        },
    }
}
