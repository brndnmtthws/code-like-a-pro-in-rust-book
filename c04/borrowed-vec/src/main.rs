fn main() {
    let vec = vec![1, 2, 3];
    let slice = vec.as_slice();
    // vec.resize(10, 0); error! does not compile because of the borrow
    // checker
    println!("{}", slice[0]);
}
