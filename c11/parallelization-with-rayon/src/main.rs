use std::time::Instant;

use rand::Rng;

fn generate_random_values(count: usize) -> Vec<i64> {
    let mut rng = rand::thread_rng();

    let mut ret = Vec::with_capacity(count);

    for _ in 0..count {
        ret.push(rng.gen_range(i64::MIN..i64::MAX));
    }

    ret
}

fn generate_random_strings(count: usize) -> Vec<String> {
    let mut rng = rand::thread_rng();

    let mut ret = Vec::with_capacity(count);

    for _ in 0..count {
        let mut s = String::new();
        for _ in 0..50000 {
            s.push(char::from_u32(rng.gen_range(0..127)).unwrap());
        }
        ret.push(s);
    }

    ret
}

fn main() {
    use rayon::prelude::*;
    use regex::Regex;

    let data = generate_random_values(1_000);

    let start = Instant::now();
    let sum = data
        .iter()
        .map(|n| n.wrapping_mul(*n))
        .reduce(|a: i64, b: i64| a.wrapping_add(b));
    let finish = Instant::now() - start;
    println!(
        "Summing squares without rayon took {}s",
        finish.as_secs_f64()
    );

    let start = Instant::now();
    let sum = data
        .par_iter()
        .map(|n| n.wrapping_mul(*n))
        .reduce(|| 0, |a: i64, b: i64| a.wrapping_add(b));
    let finish = Instant::now() - start;
    println!("Summing squares with rayon took {}s", finish.as_secs_f64());

    let data = generate_random_strings(500);
    let re = Regex::new(r"catdog").unwrap();

    let start = Instant::now();
    let matches: Vec<_> = data.iter().filter(|s| re.is_match(s)).collect();
    let finish = Instant::now() - start;
    println!("Regex took {}s", finish.as_secs_f64());

    let start = Instant::now();
    let matches: Vec<_> =
        data.par_iter().filter(|s| re.is_match(s)).collect();
    let finish = Instant::now() - start;
    println!("Regex with rayon took {}s", finish.as_secs_f64());
}
