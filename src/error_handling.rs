use std::fs::File;
use std::io::Read;

pub fn get_file_unsave() {
    let contents = read_file_content_unsave(); // This will panic if there's an error
    println!("File contents: {}", contents);
}
fn read_file_content_unsave() -> String {
    let mut file = File::open("example.txt").unwrap();  // Risky if file is not found
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();  // Panics on error
    contents
}
pub fn get_file() {
    match read_file_content() {
        Ok(contents) => println!("File contents: {}", contents),
        Err(e) => println!("Error reading file: {}", e),
    }
}
fn read_file_content() -> Result<String, std::io::Error> {
    let mut file = File::open("example.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}