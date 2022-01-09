use std::{fs::File, io::Read};

#[derive(Debug)]
struct Error(String);

impl From<std::io::Error> for Error {
    fn from(other: std::io::Error) -> Self {
        Self(other.to_string())
    }
}

fn read_file(name: &str) -> Result<String, Error> {
    let mut f = File::open(name)?;
    let mut output = String::new();

    f.read_to_string(&mut output)?;

    Ok(output)
}

fn main() {
    println!("{}", read_file("src/main.rs").unwrap()); // this line succeeds
    println!("{}", read_file("src/failure.rs").unwrap()); // this line fails
}
