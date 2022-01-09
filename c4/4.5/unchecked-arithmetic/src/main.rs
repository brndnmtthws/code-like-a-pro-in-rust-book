fn main() {
    // println!("{}", 1 / 0); does not compile

    let one = 1;
    let zero = 0;
    // println!("{}", one / zero); does not compile

    let one = 1;
    let zero = one - 1;
    // println!("{}", one / zero); still doesn't compile

    let one = { || 1 }();
    let zero = { || 0 }();
    println!("{}", one / zero); // panics here
}
