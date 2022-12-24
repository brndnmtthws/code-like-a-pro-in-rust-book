use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, Debug)]
struct CompoundKey {
    name: String,
    value: i32,
}

fn main() {
    let mut map = HashMap::<CompoundKey, String>::new();

    let key = CompoundKey {
        name: "key 1".into(),
        value: 1,
    };
    map.insert(key, "some value".into());

    println!("{:#?}", map);
}
