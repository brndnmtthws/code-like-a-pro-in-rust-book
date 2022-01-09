fn lowercased(s: String) -> String {
    s.to_lowercase()
}

fn lowercased_ascii(mut s: String) -> String {
    s.make_ascii_lowercase();
    s
}

fn main() {
    let s1 = "HeLlO";
    let s2 = "CoMpUtErS";
    println!("lowercased('{}') -> '{}'", s1, lowercased(s1.to_owned()));
    println!(
        "lowercased_ascii('{}`) -> '{}'",
        s2,
        lowercased_ascii(s2.to_owned())
    );
}
