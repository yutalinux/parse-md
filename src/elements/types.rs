use nom::{error::ParseError, IResult, InputLength};

use super::{heading::HeadingElement, text::TextElement};

pub enum ElementTypes {
    H1(HeadingElement),
    H2(HeadingElement),
    H3(HeadingElement),
    H4(HeadingElement),
    H5(HeadingElement),
    H6(HeadingElement),
    Text(TextElement),
}
