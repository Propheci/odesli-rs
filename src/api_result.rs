use std::collections::HashMap;

use crate::{EntityType, SupportedPlatform};

use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct LinkByPlatform {
    /// The unique ID for this entity. Use it to look up data about this entity
    /// at `entities_by_unique_id[entity_unique_id]`
    #[serde(rename = "entityUniqueId")]
    pub entity_unique_id: String,

    /// The URL for this match.
    #[serde(rename = "url")]
    pub url: String,

    /// The native app URI that can be used on mobile devices to open this
    /// entity directly in the native app
    #[serde(rename = "nativeAppUriMobile")]
    pub native_app_uri_mobile: Option<String>,

    /// The native app URI that can be used on desktop devices to open this
    /// entity directly in the native app
    #[serde(rename = "nativeAppUriDesktop")]
    pub native_app_uri_desktop: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct EntityByUniqueID {
    /// This is the unique identifier on the streaming platform/API provider
    #[serde(rename = "id")]
    pub id: String,

    /// Whether its a 'song' or an 'album'.
    #[serde(rename = "type")]
    pub entity_type: EntityType,

    #[serde(rename = "title")]
    pub title: Option<String>,
    #[serde(rename = "artistName")]
    pub artist_name: Option<String>,
    #[serde(rename = "thumbnailUrl")]
    pub thumbnail_url: Option<String>,
    #[serde(rename = "thumbnailWidth")]
    pub thumbnail_width: Option<u64>,
    #[serde(rename = "thumbnailHeight")]
    pub thumbnail_height: Option<u64>,

    /// The API provider that powered this match. Useful if you'd like to use
    /// this entity's data to query the API directly
    #[serde(rename = "apiProvider")]
    pub api_provider: SupportedPlatform,

    /// An array of platforms that are "powered" by this entity. E.g. an entity
    /// from Apple Music will generally have a `platforms` array of
    /// `[AppleMusic, iTunes]` since both those platforms/links are derived
    /// from this single entity
    pub platforms: Vec<SupportedPlatform>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct LinksAPIResult {
    /// The unique ID for the input entity that was supplied in the request.
    /// The data for this entity, such as title, artistName, etc. will be found
    /// at `nodes_by_unique_id[entity_unique_id]`
    #[serde(rename = "entityUniqueId")]
    pub entity_unique_id: String,

    /// The `userCountry` query param that was supplied in the request. It
    /// signals the country/availability we use to query the streaming
    /// platforms. Defaults to "US" if no `user_country` supplied in the request.
    ///
    /// NOTE: As a fallback, our service may respond with matches that were
    /// found in a locale other than the `user_country` supplied
    #[serde(rename = "userCountry")]
    pub user_country: String,

    /// A URL that will render the Songlink page for this entity
    #[serde(rename = "pageUrl")]
    pub page_url: String,

    /// Each key is a platform, and each value is a struct that contains data
    /// for linking to the match.
    #[serde(rename = "linksByPlatform")]
    pub links_by_platform: HashMap<SupportedPlatform, LinkByPlatform>,

    /// Each key is a unique identifier for a streaming entity, and each value
    /// is an object that contains data for that entity, such as `title`,
    /// `artist_name`, `thumbnail_url`, etc.
    #[serde(rename = "entitiesByUniqueId")]
    pub entities_by_unique_id: HashMap<String, EntityByUniqueID>,
}
