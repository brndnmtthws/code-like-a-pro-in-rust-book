use tokio::io::{self, AsyncWriteExt};

async fn write_file(filename: &str) -> io::Result<()> {
    let mut f = tokio::fs::File::create(filename).await?;
    f.write_all(b"Hello, file!").await?;
    f.flush().await?;

    Ok(())
}

fn read_file(filename: &str) -> io::Result<String> {
    std::fs::read_to_string(filename)
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let filename = "mixed-sync-async.txt";
    write_file(filename).await?;

    let contents =
        tokio::task::spawn_blocking(|| read_file(filename)).await??;

    println!("File contents: {}", contents);

    tokio::fs::remove_file(filename).await?;

    Ok(())
}
