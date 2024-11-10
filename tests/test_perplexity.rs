#[cfg(test)]
mod tests {
    use perplexity::{Perplexity, SonarModel};

    #[cfg(test)]
    mod test_utils {
        use tokio::runtime::Runtime;

        pub(crate) fn setup_runtime() -> Runtime {
            tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .expect("Failed to create Tokio runtime")
        }
    }

    fn main() -> Result<(), Box<dyn std::error::Error>> {
        let rt = test_utils::setup_runtime();
        rt.block_on(async {
            let perplexity = Perplexity::builder()
                // .api_key("your-api-key-here") # default env PERPLEXITY_API_KEY
                .model(SonarModel::Large)
                .build()?;

            let response = perplexity.query("What is the capital of France?").await?;
            println!("Response: {}", response);
            Ok(())
        })
    }
}
