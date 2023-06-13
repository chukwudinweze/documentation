use std::{fs::File, io::Read};

fn main() {
    match read_myfile() {
        Ok(content) => println!("File content: {}", content),
        Err(err) => println!("Error occured:{}", err),
    }
}

fn read_myfile() -> Result<String, String> {
    let mut file = match File::open("example.txt") {
        Ok(file) => file,
        Err(err) => return Err(format!("Error opening file:{}", err)),
    };

    let mut content = String::new();

    match file.read_to_string(&mut content) {
        Ok(_) => Ok(content),
        Err(err) => return Err(format!("Error reading file: {}", err)),
    }
}
