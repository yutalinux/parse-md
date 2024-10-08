mod heading;
mod text;
mod types;

use anyhow::{Context, Ok, Result};
use heading::HeadingElement;
use types::ElementTypes;

pub struct Elements {
    elements: Vec<ElementTypes>,
}

impl Elements {
    fn parse_code(s: &'static str) -> Result<Elements> {
        let (s, elm) = HeadingElement::parse(s).context("")?;

        Ok(Elements {
            elements: Vec::new(),
        })
    }

    pub fn parse(code: &'static str) -> Elements {
        Self::parse_code(code).unwrap()
    }
}
