use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    character::complete::{char, multispace0},
    IResult,
};

use super::types::ElementTypes;

pub struct HeadingElement {
    pub text: String,
}

impl HeadingElement {
    fn new(text: &str) -> HeadingElement {
        HeadingElement {
            text: text.to_string(),
        }
    }

    pub fn parse(s: &str) -> IResult<&str, ElementTypes> {
        let (s, open) = alt((
            tag("#"),
            tag("##"),
            tag("###"),
            tag("####"),
            tag("#####"),
            tag("######"),
        ))(s)?;

        let (level, close) = match open {
            "#" => (1, '\n'),
            "##" => (2, '\n'),
            "###" => (3, '\n'),
            "####" => (4, '\n'),
            "#####" => (5, '\n'),
            "######" => (6, '\n'),
            _ => panic!(),
        };
        let (s, _) = multispace0(s)?;

        let (s, text) = is_not("\n")(s)?;

        let (s, _) = char(close)(s)?;

        let elm = Self::new(text);
        let elm = match level {
            1 => ElementTypes::H1(elm),
            2 => ElementTypes::H2(elm),
            3 => ElementTypes::H3(elm),
            4 => ElementTypes::H4(elm),
            5 => ElementTypes::H5(elm),
            6 => ElementTypes::H6(elm),
            _ => panic!(),
        };

        Ok((s, elm))
    }
}

#[cfg(test)]
mod tests {
    use nom::IResult;

    use crate::elements::{heading::HeadingElement, types::ElementTypes};

    #[test]
    fn parse() {}
}
