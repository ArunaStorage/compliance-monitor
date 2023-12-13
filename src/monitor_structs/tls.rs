use crate::traits::{Monitor, ResultType};
use anyhow::Result;
use reqwest::tls::TlsInfo;

pub struct TlsMonitor {
    name: String,
    endpoint: String,
    client: reqwest::Client,
}

impl TlsMonitor {
    pub fn new(name: String, endpoint: String) -> Result<Self> {
        Ok(TlsMonitor {
            name,
            endpoint,
            client: reqwest::ClientBuilder::new().build()?,
        })
    }
}

#[async_trait::async_trait]
impl Monitor for TlsMonitor {
    async fn monitor(&mut self) -> Result<(ResultType, u64)> {
        let now = tokio::time::Instant::now();
        let counter = match self.client.get(self.endpoint.clone()).send().await {
            Ok(response) => if response.url().scheme() == "https" {
                ResultType::Counter(1)
            } else {
                ResultType::Counter(0)
            },
            Err(e) => {
                ResultType::Counter(0)
            }
        
        };
        let time = now.elapsed().as_millis() as u64;
        Ok((counter, time))
    }

    fn get_name(&self) -> &str {
        &self.name
    }

}