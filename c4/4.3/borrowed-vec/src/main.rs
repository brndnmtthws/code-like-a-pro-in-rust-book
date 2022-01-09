fn main() {
    let mut vec = vec![1, 2, 3];
    let slice = vec.as_slice();
    vec.resize(10, 0);
    println!("{}", slice[0]);
}
