#[cfg(test)]
mod tests {
    use client_perplexity::perplexity::Perplexity;

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

    fn main() {
        let rt = test_utils::setup_runtime();
        rt.block_on(async {
            let perplexity = Perplexity::new(None);
            let result = perplexity.query("What is the capital of France?").await;
            if let Ok(response) = result {
                println!("Response: {}", response);
            } else {
                println!("Error occurred");
            }
        });
    }
}
