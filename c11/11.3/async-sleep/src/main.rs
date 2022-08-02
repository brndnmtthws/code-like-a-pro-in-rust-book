fn main() {
    use std::time;

    let duration = time::Duration::from_secs(1);

    tokio::runtime::Builder::new_current_thread()
        .enable_time()
        .build()
        .unwrap()
        .block_on(async {
            tokio::time::sleep(duration).await;
            println!("Hello, world!");
        });
}
