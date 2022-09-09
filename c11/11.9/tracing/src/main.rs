use tokio::time::{sleep, Duration};

#[tracing::instrument]
async fn sleep_1s() {
    sleep(Duration::from_secs(1)).await;
}

#[tracing::instrument]
async fn sleep_2s() {
    sleep(Duration::from_secs(2)).await;
}

#[tracing::instrument]
async fn sleep_3s() {
    sleep(Duration::from_secs(3)).await;
}

#[tokio::main]
async fn main() {
    console_subscriber::init();

    loop {
        tokio::spawn(sleep_1s());
        tokio::spawn(sleep_2s());
        sleep_3s().await;
    }
}
