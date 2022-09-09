async fn sleep_1s_nonblocking(task: &str) {
    use tokio::time::{sleep, Duration};
    println!("Entering sleep_1s_nonblocking({task})");
    sleep(Duration::from_secs(1)).await;
    println!("Returning from sleep_1s_nonblocking({task})");
}

async fn sleep_1s_blocking(task: &str) {
    use std::{thread, time::Duration};
    println!("Entering sleep_1s_blocking({task})");
    thread::sleep(Duration::from_secs(1));
    println!("Returning from sleep_1s_blocking({task})");
}

#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() {
    println!("Test 1: Run 2 async tasks sequentially");
    sleep_1s_blocking("Task 1").await;
    sleep_1s_blocking("Task 2").await;

    println!("Test 2: Run 2 async tasks concurrently (same thread)");
    tokio::join!(sleep_1s_blocking("Task 3"), sleep_1s_blocking("Task 4"));

    println!("Test 3: Run 2 async tasks in parallel");
    tokio::join!(
        tokio::spawn(sleep_1s_blocking("Task 5")),
        tokio::spawn(sleep_1s_blocking("Task 6"))
    );

    println!("Test 4: Run 2 async tasks sequentially (non-blocking)");
    sleep_1s_nonblocking("Task 7").await;
    sleep_1s_nonblocking("Task 8").await;

    println!("Test 5: Run 2 async tasks concurrently (same thread, non-blocking)");
    tokio::join!(
        sleep_1s_nonblocking("Task 9"),
        sleep_1s_nonblocking("Task 10")
    );

    println!("Test 6: Run 2 async tasks in parallel (non-blocking)");
    tokio::join!(
        tokio::spawn(sleep_1s_nonblocking("Task 11")),
        tokio::spawn(sleep_1s_nonblocking("Task 12"))
    );
}
