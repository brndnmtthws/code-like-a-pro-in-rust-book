use tokio::time::{sleep, Duration};
async fn sleep_1s() {
    sleep(Duration::from_secs(1)).await;
}

#[tokio::main]
async fn main() {
    sleep_1s().await;
}

mod tests {
    use super::*;
    use tokio::time::Instant;

    #[tokio::test]
    async fn sleep_test() {
        let start_time = Instant::now();
        sleep(Duration::from_secs(1)).await;
        let end_time = Instant::now();
        let seconds = end_time
            .checked_duration_since(start_time)
            .unwrap()
            .as_secs();
        assert_eq!(seconds, 1);
    }
}
