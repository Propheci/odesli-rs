use std::str::FromStr;

use crate::OdesliError;

use clap::builder::PossibleValue;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Serialize, PartialEq)]
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

#[cfg(feature = "clap")]
impl clap::ValueEnum for EntityType {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Song, Self::Album]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(PossibleValue::new(self.as_str()))
    }
}

impl EntityType {
    pub fn as_str(&self) -> &'static str {
        match self {
            EntityType::Album => "album",
            EntityType::Song => "song",
        }
    }
}
