fn print_String(s: String) {
    println!("print_String: {}", s);
}

fn print_str(s: &str) {
    println!("print_str: {}", s);
}

fn main() {
    // let s: str = "impossible str";
    print_String(String::from("String"));
    print_str(&String::from("String"));
    print_str("str");
    print_String("str");
}
