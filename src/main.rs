use clap::Parser;
use dotenv::dotenv;
use std::env;
use std::io::Write;

mod openai_embedding;
use openai_embedding::get_embedding;
use openai_embedding::OpenAIEmbeddingResponse;

mod rag;
use rag::split_text;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Parser)]
struct Args {
    #[arg(long, short = 'p')]
    prompt: Option<String>,
    #[arg(long, short = 'e')]
    embedding: bool,
    #[arg(long, short = 'c')]
    chat: bool,
    #[arg(long, short = 'b')]
    book: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let prompt: &str = args
        .prompt
        .as_deref()
        .unwrap_or("What is so special about the Rust programming language?");

    dotenv().ok();
    let uri = env::var("OPENAI_URL").unwrap();
    let token = env::var("OPENAI_TOKEN").unwrap();
    let model = env::var("OPENAI_MODEL").unwrap();

    let book = args.book.as_deref();
    match book {
        Some(b) => {
            if args.embedding || args.chat {
                panic!("Cannot specify --embedding or --chat if --book is used");
            }
            if prompt.len() > 0 {
                println!("Ignoring prompt argument");
                let _ = std::io::stdout().flush();
            }
            let _ = split_text(b, 100, true);
            std::process::exit(0);
        }
        None => {}
    }

    match (args.embedding, args.chat) {
        (true, true) => {
            panic!("Cannot specify both --embedding and --chat as instructions");
        }
        (true, false) => {
            let response =
                get_embedding(uri.as_str(), prompt, model.as_str(), token.as_str()).await?;
            println!("\n{:?}\n", response);
            println!("\n{:?}\n", response.data);
        }
        (false, true) => {}
        (false, false) => {
            panic!("Must specify either --embedding or --chat as instructions");
        }
    }
    Ok(())
}
