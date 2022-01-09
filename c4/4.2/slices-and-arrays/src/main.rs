fn main() {
    let array = [0u8; 64];
    let slice: &[u8] = &array;

    let (first_half, second_half) = slice.split_at(32);
    println!(
        "first_half.len()={} second_half.len()={}",
        first_half.len(),
        second_half.len()
    );

    let wordlist = "one,two,three,four";
    for word in wordlist.split(',') {
        println!("word={}", word);
    }
}
