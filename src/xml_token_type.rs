#[derive(Debug, PartialEq, Eq)]
pub enum XmlTokenType{
    XmlOpeningTagWithAttributes,
    XmlOpeningTagWithoutAttributes,
    XmlOpeningTagFinish,
    XmlClosingTagWithAttributes,
    XmlClosingTagWithoutAttributes,
    XmlAttributeValue,
    XmlEqual,
    XmlText,
    XmlEnd
}