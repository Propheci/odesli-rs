use std::str::FromStr;

use crate::OdesliError;

use serde::{de, Deserialize, Deserializer};
use strum::EnumIter;

#[derive(Clone, Debug, EnumIter, Eq, Hash, PartialEq)]
/// `Platforms` as defined in the documentation.
pub enum Platform {
    Spotify,
    #[allow(non_camel_case_types)]
    iTunes,
    AppleMusic,
    YouTube,
    YouTubeMusic,
    Google,
    GoogleStore,
    Pandora,
    Deezer,
    Tidal,
    AmazonStore,
    AmazonMusic,
    SoundCloud,
    Napster,
    Yandex,
    Spinrilla,
    Audius,
    Anghami,
    Boomplay,
    Audiomack,
}

impl FromStr for Platform {
    type Err = OdesliError;

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
            _ => Err(Self::Err::UnknownPlatform(value.to_string())),
        }
    }
}

impl<'de> Deserialize<'de> for Platform {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|err| de::Error::custom(err.to_string()))
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
#[derive(Clone, Debug, EnumIter, Eq, Hash, PartialEq)]
pub enum APIProvider {
    Spotify,
    #[allow(non_camel_case_types)]
    iTunes,
    YouTube,
    Google,
    Pandora,
    Deezer,
    Tidal,
    Amazon,
    SoundCloud,
    Napster,
    Yandex,
    Spinrilla,
    Audius,
    Anghami,
    Boomplay,
    Audiomack,
}

impl FromStr for APIProvider {
    type Err = OdesliError;

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
            _ => Err(Self::Err::UnknownAPIProvider(value.to_string())),
        }
    }
}

impl<'de> Deserialize<'de> for APIProvider {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|err| de::Error::custom(err.to_string()))
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
