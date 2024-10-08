pub mod elements;

use std::io::Write;

use anyhow::Result;
use elements::Elements;

pub struct Markdown {
    code: String,
    elements: Elements,
}

impl Markdown {
    pub fn new(code: &'static str) -> Markdown {
        let elements = Elements::parse(code);

        Markdown {
            code: code.to_string(),
            elements,
        }
    }

    pub fn save<W: Write>(self, mut writer: W) -> Result<()> {
        writer.write(self.code.as_bytes())?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
