fn main() {
    let mut most_populous_us_cities = vec!["New York City", "Los Angeles", "Chicago", "Houston"];
    let most_populous_us_cities_cloned = most_populous_us_cities.clone();
    most_populous_us_cities.push("Phoenix");
    println!("most_populous_us_cities = {:#?}", most_populous_us_cities);
    println!(
        "most_populous_us_cities_cloned = {:#?}",
        most_populous_us_cities_cloned
    );
}
