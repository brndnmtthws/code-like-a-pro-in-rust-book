#[derive(Debug)]
struct Error {
    message: String,
}

impl From<std::io::Error> for Error {
    fn from(other: std::io::Error) -> Self {
        Self {
            message: other.to_string(),
        }
    }
}

fn read_file(name: &str) -> Result<String, Error> {
    use std::fs::File;
    use std::io::prelude::*;

    let mut file = File::open(name)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() {
    println!("{}", read_file("src/main.rs").unwrap());
    println!("{}", read_file("src/not-there.rs").unwrap());
}
