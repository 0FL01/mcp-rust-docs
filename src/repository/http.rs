#[async_trait::async_trait]
pub trait HttpRepository: std::fmt::Debug + Send + Sync {
    async fn get(&self, url: &str) -> Result<String, crate::error::Error>;
}

#[derive(Debug)]
pub struct HttpRepositoryImpl {}

#[async_trait::async_trait]
impl HttpRepository for HttpRepositoryImpl {
    async fn get(&self, url: &str) -> Result<String, crate::error::Error> {
        let client = crate::cache::get_or_init_reqwest_client().await?;

        let response = client.get(url).send().await.map_err(|e| {
            tracing::error!("{}", e);
            crate::error::Error::Http(e.to_string())
        })?;

        if !response.status().is_success() {
            return Err(crate::error::Error::Http(format!(
                "Failed to fetch URL {}: {}",
                url,
                response.status()
            )));
        }

        let html = response.text().await.map_err(|e| {
            tracing::error!("{}", e);
            crate::error::Error::Http(e.to_string())
        })?;

        Ok(html)
    }
}
