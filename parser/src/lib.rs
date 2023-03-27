use std::{fmt::Display, str::FromStr};

use parser::Token;

pub mod error;
mod parser;
mod rendering;

pub struct Regex {
    tok: Token,
}

impl Display for Regex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", rendering::render_token(&self.tok))
    }
}

impl FromStr for Regex {
    type Err = error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tok = parser::parse_expr(s.chars())?;

        Ok(Self { tok })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phone_number_display() {
        let re: Regex = r#"^(\+\d{1,2}\s)?\(?\d{3}\)?[a-z\s.-]\d{3}[\s.-]\d{4}$"#
            .parse()
            .expect("parse");

        format!("{re}");
    }
}
