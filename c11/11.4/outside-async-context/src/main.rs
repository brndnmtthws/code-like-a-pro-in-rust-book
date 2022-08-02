use tokio::task::JoinHandle;

fn not_an_async_function() -> JoinHandle<()> {
    tokio::task::spawn(async {
        println!("Second print statement");
    })
}

#[tokio::main]
async fn main() {
    println!("First print statement");
    not_an_async_function().await.ok();
}
