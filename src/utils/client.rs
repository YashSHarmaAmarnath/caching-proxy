use reqwest::{Client, Method};
use std::time::Duration;

pub struct Forwarder {
    client: Client,
}

#[derive(Debug)]
pub struct ForwardedResponse {
    pub status: u16,
    pub headers: reqwest::header::HeaderMap,
    pub body: Vec<u8>,
}

impl Forwarder {
    pub fn new() -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .expect("failed to build HTTP client");

        Forwarder { client }
    }

    pub async fn forward(
        &self,
        origin: &str,
        method: Method,
        path_and_query: &str,
        body: Option<Vec<u8>>,
    ) -> Result<ForwardedResponse, String> {
        let url = format!("{}{}", origin.trim_end_matches("/"), path_and_query);

        let mut request = self.client.request(method, &url);

        if let Some(b) = body {
            request = request.body(b);
        }

        let response = request
            .send()
            .await
            .map_err(|e| format!("failed to send request error: {e}"))?;

        let status = response.status().as_u16();

        let headers = response.headers().clone();

        let body = response
            .bytes()
            .await
            .map_err(|e| format!("failed to read origin response body: {e}"))?
            .to_vec();

        Ok(ForwardedResponse {
            status,
            headers,
            body,
        })
    }
}
