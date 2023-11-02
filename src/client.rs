use crate::{
    EntityType, LinksAPIResult, OdesliError, Platform, API_VERSION, BASE_URL, LINKS_ENDPOINT,
};

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

    /// Modify the API version being used.
    ///
    /// Currently, the only version available is `"v1-alpha.1"`.
    pub fn with_api_version(mut self, version: String) -> Self {
        self.api_version = version;
        self
    }

    /// Modify the [`reqwest::Client`] being used for making the calls.
    pub fn with_http_client(mut self, client: reqwest::Client) -> Self {
        self.http_client = client;
        self
    }

    /// Build and return the [`crate::OdesliClient`] with the configuration set.
    pub fn build(self) -> OdesliClient {
        OdesliClient {
            api_key: self.api_key,
            api_url: format!("{}/{}", BASE_URL, self.api_version),
            http_client: self.http_client,
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
    api_key: Option<String>,
    api_url: String,
    http_client: reqwest::Client,
}

impl OdesliClient {
    async fn get(&self, mut params: Vec<(&str, &str)>) -> Result<LinksAPIResult, OdesliError> {
        if let Some(key) = self.api_key.as_ref() {
            params.push(("key", key.as_str()));
        }

        let api_endpoint = format!("{}/{}", self.api_url, LINKS_ENDPOINT);

        match self
            .http_client
            .get(api_endpoint)
            .query(params.as_slice())
            .send()
            .await
        {
            Ok(res) => {
                let status_code = res.status();
                let body = res.text().await.unwrap();

                if status_code.as_u16() != 200 {
                    return Err(OdesliError::Non200StatusCode { status_code, body });
                }

                serde_json::from_str::<LinksAPIResult>(&body).map_err(|err| {
                    OdesliError::ParseError {
                        error: err.to_string(),
                        body,
                    }
                })
            }
            Err(err) => Err(OdesliError::ReqwestError(err)),
        }
    }

    /// Get a song/album by using its platform specific URL.
    ///
    /// # Arguments
    ///
    /// * `url`: The URL of the song/album to get
    pub async fn get_by_url(&self, url: &str) -> Result<LinksAPIResult, OdesliError> {
        self.get(vec![("url", url)]).await
    }

    /// Get a song/album by using its platform specific ID.
    ///
    /// # Arguments
    ///
    /// * `id`: The ID of the entity in the corresponding platform.
    /// * `platform`: The platform identifier (using [`crate::Platform`]).
    /// * `entity_type`: The entity type of the ID ([`crate::EntityType`]).
    pub async fn get_by_id(
        &self,
        id: &str,
        platform: &Platform,
        entity_type: &EntityType,
    ) -> Result<LinksAPIResult, OdesliError> {
        self.get(vec![
            ("id", id),
            ("platform", platform.as_str()),
            ("type", entity_type.as_str()),
        ])
        .await
    }
}
