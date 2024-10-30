#[cfg(test)]
mod tests {
    use client_perplexity::perplexity::Perplexity;

    #[cfg(test)]
    mod test_utils {

        #[cfg(test)]
        use tokio;

        #[tokio::test]
        async fn async_test() {
            // Your async test code here
        }

        use tokio::runtime::Runtime;

        pub(crate) fn setup_runtime() -> Runtime {
            tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .expect("Failed to create Tokio runtime")
        }
    }

    #[test]
    fn test_perplexity_query() {
        let rt = test_utils::setup_runtime();
        rt.block_on(async {
            let perplexity = Perplexity::new(None);
            let result = perplexity.query("What is the capital of France?").await;
            assert!(result.is_ok());
            let response = result.unwrap();
            assert!(response.contains("Paris"));
        });
    }
}
