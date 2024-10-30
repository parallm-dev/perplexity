use perplexity::{Perplexity, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let client = Perplexity::builder()
        // .api_key("your-api-key-here") # default env PERPLEXITY_API_KEY
        .model("custom-model-name")
        .build()?;

    let response = client.query("What is the capital of France?").await?;
    println!("Response: {}", response);

    Ok(())
}
