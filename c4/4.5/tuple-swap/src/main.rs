fn swap<A, B>(a: A, b: B) -> (B, A) {
    (b, a)
}

fn main() {
    let a = 1;
    let b = 2;

    println!("{:?}", swap(a, b));
}
