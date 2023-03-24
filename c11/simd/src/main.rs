#![feature(portable_simd, array_zip)]

fn initialize() -> ([u64; 64], [u64; 64]) {
    let mut a = [0u64; 64];
    let mut b = [0u64; 64];
    (0..64).for_each(|n| {
        a[n] = u64::try_from(n).unwrap();
        b[n] = u64::try_from(n + 1).unwrap();
    });
    (a, b)
}

fn main() {
    use std::simd::Simd;
    use std::time::Instant;

    let (mut a, b) = initialize();

    // perform some calculations using normal math
    let now = Instant::now();
    for _ in 0..100_000 {
        let c = a.zip(b).map(|(l, r)| l * r);
        let d = a.zip(c).map(|(l, r)| l + r);
        let e = c.zip(d).map(|(l, r)| l * r);
        a = e.zip(d).map(|(l, r)| l ^ r);
    }
    println!("Without SIMD took {}s", now.elapsed().as_secs_f32());

    let (a_vec, b_vec) = initialize();

    let mut a_vec = Simd::from(a_vec);
    let b_vec = Simd::from(b_vec);

    // perform the same calculations with SIMD
    let now = Instant::now();
    for _ in 0..100_000 {
        let c_vec = a_vec * b_vec;
        let d_vec = a_vec + c_vec;
        let e_vec = c_vec * d_vec;
        a_vec = e_vec ^ d_vec;
    }
    println!("With SIMD took {}s", now.elapsed().as_secs_f32());

    // check the final result is the same in both
    assert_eq!(&a, a_vec.as_array());
}
