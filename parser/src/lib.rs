use std::{fmt::Display, str::FromStr};

pub mod error;
mod parser;

pub struct Regex {}

impl Display for Regex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl FromStr for Regex {
    type Err = error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tok = parser::parse_expr(s.chars());
        todo!()
    }
}
