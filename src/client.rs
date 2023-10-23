use std::sync::{Arc, Mutex};

use crate::{EntityType, LinksAPIResult, Platform, API_VERSION, BASE_URL, LINKS_ENDPOINT};

#[derive(Clone)]
struct InnerClient {
    api_key: Option<String>,
    api_url: String,
    http_client: reqwest::Client,
}

/// Helper to build a client for Odesli. You can modify things like the `api_key`,
/// `api_version` and the inner `http_client` using the builder.
pub struct ClientBuilder {
    api_key: Option<String>,
    api_version: String,
    http_client: reqwest::Client,
}

impl ClientBuilder {
    /// Modify the Odesli API key being used in the API calls.
    pub fn with_api_key(mut self, key: String) -> Self {
        self.api_key = Some(key);
        self
    }

    /// Modify the API version being used. Currently, the only version available
    /// is `"v1-alpha.1"`.
    pub fn with_api_version(mut self, version: String) -> Self {
        self.api_version = version;
        self
    }

    /// Modify the `reqwest::Client` being used for making the calls.
    pub fn with_http_client(mut self, client: reqwest::Client) -> Self {
        self.http_client = client;
        self
    }

    /// Build and return the `OdesliClient` with the configuration set.
    pub fn build(self) -> OdesliClient {
        OdesliClient {
            inner: Arc::new(Mutex::new(InnerClient {
                api_key: self.api_key,
                api_url: format!("{}/{}", BASE_URL, self.api_version),
                http_client: self.http_client,
            })),
        }
    }
}

impl Default for ClientBuilder {
    fn default() -> Self {
        Self {
            api_key: None,
            api_version: String::from(API_VERSION),
            http_client: reqwest::Client::default(),
        }
    }
}

#[derive(Clone)]
pub struct OdesliClient {
    inner: Arc<Mutex<InnerClient>>,
}

impl OdesliClient {
    async fn get(&self, mut params: Vec<(&str, &str)>) -> Result<LinksAPIResult, String> {
        let inner = self
            .inner
            .lock()
            .map_err(|err| format!("failed to lock Odesli client: {}", err))?;

        if let Some(key) = inner.api_key.as_ref() {
            params.push(("key", key.as_str()));
        }

        let api_endpoint = format!("{}/{}", inner.api_url, LINKS_ENDPOINT);

        match inner
            .http_client
            .get(api_endpoint)
            .query(params.as_slice())
            .send()
            .await
        {
            Ok(res) => {
                let status_code = res.status();
                if status_code.as_u16() >= 400 && status_code.as_u16() <= 499 {
                    return Err(format!(
                        "received non-2xx status code from Odesli: {}",
                        status_code,
                    ));
                }

                res.json::<LinksAPIResult>().await.map_err(|err| {
                    format!(
                        "failed to JSON parse response into LinksAPIResult struct: {}",
                        err
                    )
                })
            }
            Err(err) => Err(format!("failed to make request to Odesli API: {}", err)),
        }
    }

    /// Get a song/album by using its platform specific URL.
    ///
    /// `url`: The URL of the song/album to get
    pub async fn get_by_url(&self, url: &str) -> Result<LinksAPIResult, String> {
        self.get(vec![("url", url)]).await
    }

    /// Get a song/album by using its platform specific ID.
    ///
    /// `id`: The ID of the entity in the platform.
    /// `platform`: The platform.
    /// `entity_type`: Song/Album.
    pub async fn get_by_id(
        &self,
        id: &str,
        platform: &Platform,
        entity_type: &EntityType,
    ) -> Result<LinksAPIResult, String> {
        self.get(vec![
            ("id", id),
            ("platform", platform.as_str()),
            ("type", entity_type.as_str()),
        ])
        .await
    }
}
