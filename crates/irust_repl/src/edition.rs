use std::str::FromStr;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy)]
pub enum Edition {
    E2015,
    E2018,
    E2021,
}
impl Default for Edition {
    fn default() -> Self {
        Edition::E2021
    }
}

impl FromStr for Edition {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "2015" => Ok(Edition::E2015),
            "2018" => Ok(Edition::E2018),
            "2021" => Ok(Edition::E2021),
            _ => Err("Unknown edition".into()),
        }
    }
}
impl std::fmt::Display for Edition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Edition::E2015 => write!(f, "2015"),
            Edition::E2018 => write!(f, "2018"),
            Edition::E2021 => write!(f, "2021"),
        }
    }
}
