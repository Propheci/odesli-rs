use std::str::FromStr;

use crate::OdesliError;

use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub enum EntityType {
    #[serde(rename = "album")]
    Album,
    #[serde(rename = "song")]
    Song,
}

impl FromStr for EntityType {
    type Err = OdesliError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "album" => Ok(Self::Album),
            "song" => Ok(Self::Song),
            _ => Err(Self::Err::UnknownEntityType(s.to_string())),
        }
    }
}

impl EntityType {
    pub fn as_str(&self) -> &str {
        match self {
            EntityType::Album => "album",
            EntityType::Song => "song",
        }
    }
}
