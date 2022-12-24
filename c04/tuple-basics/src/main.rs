fn main() {
    let tuple = (1, 2, 3);
    println!("tuple = ({}, {}, {})", tuple.0, tuple.1, tuple.2);

    match tuple {
        (one, two, three) => println!("{}, {}, {}", one, two, three),
    }

    let (one, two, three) = tuple;
    println!("{}, {}, {}", one, two, three);
}
