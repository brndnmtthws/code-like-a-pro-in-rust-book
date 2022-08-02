#[tokio::main]
async fn main() {
    async {
        println!("This line prints first");
    }
    .await;
    let _future = async {
        println!("This line never prints");
    };
    tokio::task::spawn(async {
        println!(
            "This line prints sometimes, depending on how quick it runs"
        )
    });

    println!("This line always prints, but it may or may not be last");
}
