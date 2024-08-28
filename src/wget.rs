pub mod wget {
    use tokio;
    use reqwest::Error;
    
    pub async fn fetch_url(url: String) -> Result<String, Error> {
        let response = reqwest::get(&url).await?;
        let body = response.text().await?;
        Ok(body)
    }

    #[tokio::test]
    pub async fn get_url() {
        let result = fetch_url("https://ejangi.com".to_string()).await.unwrap();
        assert!(result.len() > 0);
    }
}