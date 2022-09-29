use std::time::Instant;

fn main() {
    let mut empty_vec = Vec::<i32>::new();
    (0..10).for_each(|v| {
        println!(
            "empty_vec has {} elements with capacity {}",
            empty_vec.len(),
            empty_vec.capacity()
        );
        empty_vec.push(v)
    });

    let big_vec = vec![0; 10_000_000];
    let now = Instant::now();
    for i in big_vec {
        if i < 0 {
            println!("this never prints");
        }
    }
    println!("First loop took {}s", now.elapsed().as_secs_f32());

    let big_vec = vec![0; 10_000_000];
    let now = Instant::now();
    big_vec.iter().for_each(|i| {
        if *i < 0 {
            println!("this never prints");
        }
    });
    println!("Second loop took {}s", now.elapsed().as_secs_f32());

    let big_vec = vec![0; 10_000_000];
    let now = Instant::now();
    big_vec.into_iter().for_each(|i| {
        if i < 0 {
            println!("this never prints");
        }
    });
    println!("Third loop took {}s", now.elapsed().as_secs_f32());

    let big_vec_source = vec![0; 10_000_000];
    let mut big_vec_target = Vec::<i32>::with_capacity(10_000_000);
    let now = Instant::now();
    big_vec_source
        .into_iter()
        .for_each(|i| big_vec_target.push(i));
    println!("Naive copy took {}s", now.elapsed().as_secs_f32());

    let big_vec_source = vec![0; 10_000_000];
    let mut big_vec_target = vec![0; 10_000_000];
    let now = Instant::now();
    big_vec_target.copy_from_slice(&big_vec_source);
    println!("Fast copy took {}s", now.elapsed().as_secs_f32());
}
