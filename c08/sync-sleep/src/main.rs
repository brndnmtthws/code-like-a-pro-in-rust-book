fn main() {
    use std::{thread, time};

    let duration = time::Duration::from_secs(1);

    thread::sleep(duration);

    println!("Hello, world!");
}
