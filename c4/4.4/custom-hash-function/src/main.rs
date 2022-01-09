fn main() {
    use metrohash::MetroBuildHasher;
    use std::collections::HashMap;

    let mut map = HashMap::<String, String, MetroBuildHasher>::default();
    map.insert("hello?".into(), "Hello!".into());

    println!("{:?}", map.get("hello?"));
}
