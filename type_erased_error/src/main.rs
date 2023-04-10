use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn read_file_to_string<P: AsRef<Path>>(path: P) -> Result<String, Box<dyn Error>> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn main() -> Result<(), Box<dyn Error>> {
    let content = read_file_to_string("example.txt")?;
    println!("File content: {}", content);
    Ok(())
}
