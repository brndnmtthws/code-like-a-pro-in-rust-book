struct StringWrapper(String);

impl From<&str> for StringWrapper {
    fn from(other: &str) -> Self {
        Self(other.into())
    }
}

fn main() {
    println!("{}", StringWrapper::from("Hello, world!").0);
}
