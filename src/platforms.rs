use std::str::FromStr;

use serde::Deserialize;

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq)]
/// `Platforms` as defined in the documentation.
pub enum Platform {
    #[serde(rename = "spotify")]
    Spotify,
    #[serde(rename = "itunes")]
    #[allow(non_camel_case_types)]
    iTunes,
    #[serde(rename = "appleMusic")]
    AppleMusic,
    #[serde(rename = "youtube")]
    YouTube,
    #[serde(rename = "youtubeMusic")]
    YouTubeMusic,
    #[serde(rename = "google")]
    Google,
    #[serde(rename = "googleStore")]
    GoogleStore,
    #[serde(rename = "pandora")]
    Pandora,
    #[serde(rename = "deezer")]
    Deezer,
    #[serde(rename = "tidal")]
    Tidal,
    #[serde(rename = "amazonStore")]
    AmazonStore,
    #[serde(rename = "amazonMusic")]
    AmazonMusic,
    #[serde(rename = "soundcloud")]
    SoundCloud,
    #[serde(rename = "napster")]
    Napster,
    #[serde(rename = "yandex")]
    Yandex,
    #[serde(rename = "spinrilla")]
    Spinrilla,
    #[serde(rename = "audius")]
    Audius,
    #[serde(rename = "anghami")]
    Anghami,
    #[serde(rename = "boomplay")]
    Boomplay,
    #[serde(rename = "audiomack")]
    Audiomack,
}

impl FromStr for Platform {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "spotify" => Ok(Self::Spotify),
            "itunes" => Ok(Self::iTunes),
            "appleMusic" => Ok(Self::AppleMusic),
            "youtube" => Ok(Self::YouTube),
            "youtubeMusic" => Ok(Self::YouTubeMusic),
            "google" => Ok(Self::Google),
            "googleStore" => Ok(Self::GoogleStore),
            "pandora" => Ok(Self::Pandora),
            "deezer" => Ok(Self::Deezer),
            "tidal" => Ok(Self::Tidal),
            // "amazon" => Ok(Self::Amazon),
            "amazonStore" => Ok(Self::AmazonStore),
            "amazonMusic" => Ok(Self::AmazonMusic),
            "soundcloud" => Ok(Self::SoundCloud),
            "napster" => Ok(Self::Napster),
            "yandex" => Ok(Self::Yandex),
            "spinrilla" => Ok(Self::Spinrilla),
            "audius" => Ok(Self::Audius),
            "anghami" => Ok(Self::Anghami),
            "boomplay" => Ok(Self::Boomplay),
            "audiomack" => Ok(Self::Audiomack),
            _ => Err(format!("unknown platform: {}", value)),
        }
    }
}

impl Platform {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Spotify => "spotify",
            Self::iTunes => "itunes",
            Self::AppleMusic => "appleMusic",
            Self::YouTube => "youtube",
            Self::YouTubeMusic => "youtubeMusic",
            Self::Google => "google",
            Self::GoogleStore => "googleStore",
            Self::Pandora => "pandora",
            Self::Deezer => "deezer",
            Self::Tidal => "tidal",
            // Self::Amazon => "amazon",
            Self::AmazonStore => "amazonStore",
            Self::AmazonMusic => "amazonMusic",
            Self::SoundCloud => "soundcloud",
            Self::Napster => "napster",
            Self::Yandex => "yandex",
            Self::Spinrilla => "spinrilla",
            Self::Audius => "audius",
            Self::Anghami => "anghami",
            Self::Boomplay => "boomplay",
            Self::Audiomack => "audiomack",
        }
    }
}

/// `APIProviders` as defined in the documentation.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq)]
pub enum APIProvider {
    #[serde(rename = "spotify")]
    Spotify,
    #[serde(rename = "itunes")]
    #[allow(non_camel_case_types)]
    iTunes,
    #[serde(rename = "youtube")]
    YouTube,
    #[serde(rename = "google")]
    Google,
    #[serde(rename = "pandora")]
    Pandora,
    #[serde(rename = "deezer")]
    Deezer,
    #[serde(rename = "tidal")]
    Tidal,
    #[serde(rename = "amazon")]
    Amazon,
    #[serde(rename = "soundcloud")]
    SoundCloud,
    #[serde(rename = "napster")]
    Napster,
    #[serde(rename = "yandex")]
    Yandex,
    #[serde(rename = "spinrilla")]
    Spinrilla,
    #[serde(rename = "audius")]
    Audius,
    #[serde(rename = "anghami")]
    Anghami,
    #[serde(rename = "boomplay")]
    Boomplay,
    #[serde(rename = "audiomack")]
    Audiomack,
}

impl FromStr for APIProvider {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "spotify" => Ok(Self::Spotify),
            "itunes" => Ok(Self::iTunes),
            "youtube" => Ok(Self::YouTube),
            "google" => Ok(Self::Google),
            "pandora" => Ok(Self::Pandora),
            "deezer" => Ok(Self::Deezer),
            "tidal" => Ok(Self::Tidal),
            "amazon" => Ok(Self::Amazon),
            "soundcloud" => Ok(Self::SoundCloud),
            "napster" => Ok(Self::Napster),
            "yandex" => Ok(Self::Yandex),
            "spinrilla" => Ok(Self::Spinrilla),
            "audius" => Ok(Self::Audius),
            "anghami" => Ok(Self::Anghami),
            "boomplay" => Ok(Self::Boomplay),
            "audiomack" => Ok(Self::Audiomack),
            _ => Err(format!("unknown platform: {}", value)),
        }
    }
}

impl APIProvider {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Spotify => "spotify",
            Self::iTunes => "itunes",
            Self::YouTube => "youtube",
            Self::Google => "google",
            Self::Pandora => "pandora",
            Self::Deezer => "deezer",
            Self::Tidal => "tidal",
            Self::Amazon => "amazon",
            Self::SoundCloud => "soundcloud",
            Self::Napster => "napster",
            Self::Yandex => "yandex",
            Self::Spinrilla => "spinrilla",
            Self::Audius => "audius",
            Self::Anghami => "anghami",
            Self::Boomplay => "boomplay",
            Self::Audiomack => "audiomack",
        }
    }
}
