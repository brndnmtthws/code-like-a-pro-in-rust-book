use clap::{Parser, Subcommand};
use colored_json::prelude::*;
use hyper::{
    body::HttpBody as _, header::CONTENT_TYPE, Body, Client, Method,
    Request, Uri,
};
use serde_json::json;
use yansi::Paint;

#[derive(Parser)]
struct Cli {
    /// Base URL of API service
    url: hyper::Uri,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// List all todos
    List,
    /// Create a new todo
    Create {
        /// The todo body
        body: String,
    },
    /// Read a todo
    Read {
        /// The todo ID
        id: i64,
    },

    /// Update a todo
    Update {
        /// The todo ID
        id: i64,
        /// The todo body
        body: String,
        /// Mark todo as completed
        #[arg(short, long)]
        completed: bool,
    },
    /// Delete a todo
    Delete {
        /// The todo ID
        id: i64,
    },
}

async fn request(
    url: hyper::Uri,
    method: Method,
    body: Option<String>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = Client::new();

    let mut res = client
        .request(
            Request::builder()
                .uri(url)
                .method(method)
                .header("Content-Type", "application/json")
                .body(body.map(Body::from).unwrap_or_else(Body::empty))?,
        )
        .await?;

    let mut buf = Vec::new();
    while let Some(next) = res.data().await {
        let chunk = next?;
        buf.extend_from_slice(&chunk);
    }
    let s = String::from_utf8(buf)?;

    eprintln!("Status: {}", Paint::green(res.status()));
    if res.headers().contains_key(CONTENT_TYPE) {
        let content_type = res.headers()[CONTENT_TYPE].to_str()?;
        eprintln!("Content-Type: {}", Paint::green(content_type));
        if content_type.starts_with("application/json") {
            println!("{}", &s.to_colored_json_auto()?);
        } else {
            println!("{}", &s);
        }
    } else {
        println!("{}", &s);
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let cli = Cli::parse();

    let mut uri_builder = Uri::builder();
    if let Some(scheme) = cli.url.scheme() {
        uri_builder = uri_builder.scheme(scheme.clone());
    }
    if let Some(authority) = cli.url.authority() {
        uri_builder = uri_builder.authority(authority.clone());
    }

    match cli.command {
        Commands::List => {
            request(
                uri_builder.path_and_query("/v1/todos").build()?,
                Method::GET,
                None,
            )
            .await
        }
        Commands::Delete { id } => {
            request(
                uri_builder
                    .path_and_query(format!("/v1/todos/{}", id))
                    .build()?,
                Method::DELETE,
                None,
            )
            .await
        }
        Commands::Read { id } => {
            request(
                uri_builder
                    .path_and_query(format!("/v1/todos/{}", id))
                    .build()?,
                Method::GET,
                None,
            )
            .await
        }
        Commands::Create { body } => {
            request(
                uri_builder.path_and_query("/v1/todos").build()?,
                Method::POST,
                Some(json!({ "body": body }).to_string()),
            )
            .await
        }
        Commands::Update {
            id,
            body,
            completed,
        } => {
            request(
                uri_builder
                    .path_and_query(format!("/v1/todos/{}", id))
                    .build()?,
                Method::PUT,
                Some(
                    json!({"body":body,"completed":completed}).to_string(),
                ),
            )
            .await
        }
    }
}
