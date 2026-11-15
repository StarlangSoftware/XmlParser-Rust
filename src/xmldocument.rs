use std::{fs};
use crate::xmlattribute::XmlAttribute;
use crate::xmlelement::XmlElement;
use crate::xmltexttype::XmlTextType;
use crate::xmltexttype::XmlTextType::{XmlTextAttribute, XmlTextValue};
use crate::xmltokentype::XmlTokenType;
use crate::xmltokentype::XmlTokenType::{XmlAttributeValue, XmlClosingTagWithAttributes, XmlClosingTagWithoutAttributes, XmlEnd, XmlEqual, XmlOpeningTagFinish, XmlOpeningTagWithAttributes, XmlOpeningTagWithoutAttributes, XmlText};

pub struct XmlDocument{
    root: Option<usize>,
    last_read_token_type: XmlTokenType,
    data: String,
    position: usize,
    arena: Vec<XmlElement>,
    next_char: char
}

impl XmlDocument{
    /// Creates an empty xml document.
    ///
    /// # Arguments
    ///
    /// * `file_name`: Name of the xml file
    ///
    /// returns: XmlDocument
    ///
    pub fn new(file_name: String) -> Self{
        let data = fs::read_to_string(file_name);
        Self{root: None, last_read_token_type: XmlEnd, data: data.unwrap(), position:0, arena: Vec::new(), next_char: '\0'}
    }

    fn read_char(&mut self)->Option<char>{
        if self.position < self.data.as_bytes().len(){
            let ch = self.data[self.position..].chars().next()?;
            self.position += ch.len_utf8();
            Some(ch)
        } else {
            self.last_read_token_type = XmlEnd;
            None
        }
    }

    /// Reads a token character by character from xml file.
    ///
    /// # Arguments
    ///
    /// * `previous_char`: Previous character read
    /// * `extra_allowed`: If true, space or slash is allowed in the token, otherwise it is not allowed
    /// * `quota_allowed`: If true, quota is allowed in the token, otherwise it is not allowed
    ///
    /// returns: String Token read
    ///
    fn read_token(&mut self, previous_char: char, extra_allowed: bool, quota_allowed: bool)->String{
        let mut ch = previous_char;
        let mut buffer = "".to_string();
        while (ch != '\'' || extra_allowed) && (ch != '\"' || quota_allowed) && (ch != '=' || quota_allowed) && (ch != ' ' || extra_allowed) && (ch != '/' || extra_allowed) && ch != '<' && (ch != '>' || quota_allowed){
            buffer.push(ch);
            let read = self.read_char();
            if read == None{
                break;
            } else {
                ch = read.unwrap();
            }
        }
        self.next_char = ch;
        buffer
    }

    fn read_token_no_extra_no_quota(&mut self, previous_char: char)->String{
        self.read_token(previous_char, false, false)
    }

    fn read_token_no_quota(&mut self, previous_char: char, extra_allowed: bool)->String{
        self.read_token(previous_char, extra_allowed, false)
    }

    /// Parses a tag like mytag or /mytag
    ///
    /// returns: String Token read
    ///
    fn parse_tag(&mut self)->String{
        let mut ch = self.read_char();
        if ch == Some('/'){
            self.last_read_token_type = XmlClosingTagWithoutAttributes;
            ch = self.read_char();
        } else {
            self.last_read_token_type = XmlOpeningTagWithAttributes;
        }
        let token = self.read_token_no_extra_no_quota(ch.unwrap());
        ch = Option::from(self.next_char);
        if ch == Option::from('>') && self.last_read_token_type == XmlOpeningTagWithAttributes{
            self.last_read_token_type = XmlOpeningTagWithoutAttributes;
        }
        if self.last_read_token_type == XmlClosingTagWithoutAttributes && ch != Option::from('>'){
            self.last_read_token_type = XmlEnd;
            "".to_string()
        } else {
            token
        }
    }

    /// Parses an attribute value like "attribute value" or 'attribute value'
    ///
    /// returns: String Attribute value read
    ///
    fn parse_attribute_value(&mut self)->String{
        let mut ch = self.read_char();
        if ch == Some('"'){
            self.last_read_token_type = XmlAttributeValue;
            return "".to_string();
        }
        let token = self.read_token_no_quota(ch.unwrap(), true);
        ch = Option::from(self.next_char);
        if ch != Option::from('"'){
            self.last_read_token_type = XmlEnd;
            return "".to_string();
        }
        self.last_read_token_type = XmlAttributeValue;
        token
    }

    /// Parses a tag like closing tag
    ///
    /// returns: ""
    ///
    fn parse_empty_tag(&mut self)->String{
        let ch = self.read_char();
        if ch.unwrap() != '>'{
            self.last_read_token_type = XmlEnd;
        } else {
            self.last_read_token_type = XmlClosingTagWithAttributes;
        }
        "".to_string()
    }

    fn get_next_token(&mut self, xml_text_type: XmlTextType)->String{
        let mut ch = self.read_char();
        if ch.is_none(){
            return "".to_string();
        }
        while ch.unwrap() == ' ' || ch.unwrap() == '\t' || ch.unwrap() == '\n'{
            ch = self.read_char();
            if ch.is_none(){
                return "".to_string();
            }
        }
        match ch.unwrap(){
            '<' => {
                return self.parse_tag();
            }
            '\"' => {
                if xml_text_type == XmlTextValue{
                    let token = self.read_token(ch.unwrap(), true, true);
                    self.last_read_token_type = XmlText;
                    self.position = self.position - ch.unwrap().len_utf8();
                    return token
                } else {
                    return self.parse_attribute_value();
                }
            }
            '/' => {
                self.parse_empty_tag();
            }
            '=' => {
                if xml_text_type == XmlTextValue{
                    let token = self.read_token(ch.unwrap(), true, true);
                    self.last_read_token_type = XmlText;
                    self.position = self.position - ch.unwrap().len_utf8();
                    return token
                } else {
                    self.last_read_token_type = XmlEqual;
                }
            }
            '>' => {
                if xml_text_type == XmlTextValue{
                    let token = self.read_token(ch.unwrap(), true, true);
                    self.last_read_token_type = XmlText;
                    self.position = self.position - ch.unwrap().len_utf8();
                    return token
                } else {
                    self.last_read_token_type = XmlOpeningTagFinish;
                }
            }
            _ => {
                let token;
                if xml_text_type == XmlTextValue{
                    token = self.read_token(ch.unwrap(), true, true);
                } else {
                    token = self.read_token_no_quota(ch.unwrap(), true);
                }
                self.last_read_token_type = XmlText;
                self.position = self.position - ch.unwrap().len_utf8();
                return token;
            }
        }
        "".to_string()
    }

    pub fn parse(&mut self) -> bool{
        let mut text_type = XmlTextAttribute;
        let mut sibling_closed = false;
        let mut parent : Option<usize> = None;
        let mut sibling : Option<usize> = None;
        let mut current : Option<usize> = None;
        let mut xml_attribute : Option<XmlAttribute> = None;
        let mut token = self.get_next_token(text_type);
        let mut result = false;
        while self.last_read_token_type != XmlEnd{
            match self.last_read_token_type{
                XmlOpeningTagWithAttributes | XmlOpeningTagWithoutAttributes => {
                    self.arena.push(XmlElement::new(token.clone(), parent));
                    current = Some(self.arena.len() - 1);
                    if parent.is_some() {
                        if sibling.is_some() && sibling_closed{
                            self.arena[sibling.unwrap()].set_next_sibling(current.unwrap());
                            sibling = current;
                        } else {
                            self.arena[parent.unwrap()].set_first_child(current.unwrap());
                        }
                    } else {
                        if self.root.is_none() {
                            self.root = current;
                            result = true;
                        }
                    }
                    parent = current;
                    sibling_closed = false;
                    if self.last_read_token_type == XmlOpeningTagWithAttributes{
                        text_type = XmlTextAttribute;
                    } else {
                        text_type = XmlTextValue;
                    }
                }
                XmlOpeningTagFinish => {
                    text_type = XmlTextValue;
                    sibling_closed = false;
                }
                XmlClosingTagWithAttributes => {
                    sibling = current;
                    parent = self.arena[current.unwrap()].get_parent();
                    text_type = XmlTextValue;
                    sibling_closed = true;
                }
                XmlClosingTagWithoutAttributes => {
                    if token == *(self.arena[current.unwrap()].get_name()){
                        sibling = current;
                        parent = self.arena[current.unwrap()].get_parent();
                    } else {
                        if token == *self.arena[self.arena[current.unwrap()].get_parent().unwrap()].get_name(){
                            sibling = parent;
                            parent = self.arena[self.arena[current.unwrap()].get_parent().unwrap()].get_parent();
                            current = self.arena[current.unwrap()].get_parent();
                        }
                    }
                    sibling_closed = true;
                    text_type = XmlTextValue;
                }
                XmlAttributeValue => {
                    if token.is_empty() {
                        if let Some(attribute) = xml_attribute.as_mut() {
                            attribute.set_value("".to_string());
                        }
                    } else {
                        let new_data = Self::replace_escape_characters(token);
                        xml_attribute.as_mut().unwrap().set_value(new_data);
                    }
                    if let Some(attribute) = xml_attribute.take() {
                        self.arena[current.unwrap()].add_attribute(attribute);
                    }
                    text_type = XmlTextAttribute;
                }
                XmlEqual => {
                    text_type = XmlTextType::XmlTextNotAvailable;
                }
                XmlText => {
                    if text_type == XmlTextAttribute{
                        xml_attribute = Some(XmlAttribute::new(token));
                    } else {
                        if text_type == XmlTextValue{
                            let new_data = Self::replace_escape_characters(token);
                            self.arena[current.unwrap()].set_pc_data(new_data);
                        }
                    }
                }
                _ => {}
            }
            token = self.get_next_token(text_type);
        }
        result
    }

    pub fn get_first_child(&self) -> Option<&XmlElement> {
        if self.root.is_some() {
            return Some(&self.arena[0])
        }
        None
    }

    fn replace_escape_characters(token: String) -> String {
        let mut result = token.clone();
        result = result.replace("&quot;", "\"");
        result = result.replace("&amp;", "&");
        result = result.replace("&lt;", "<");
        result = result.replace("&gt;", ">");
        result = result.replace("&apos;", "'");
        result
    }

    pub fn get_xml_element(&self, index: usize) -> &XmlElement {
        &self.arena[index]
    }

}

#[cfg(test)]
mod tests {
    use crate::xmldocument::XmlDocument;

    #[test]
    fn test_document() {
        let mut doc = XmlDocument::new("test.xml".to_string());
        doc.parse();
        let root = doc.get_first_child().unwrap();
        assert_eq!(root.get_name(), "frameset");
        let first_child = doc.get_xml_element(root.get_first_child().unwrap());
        assert_eq!(first_child.get_name(), "role");
        let pc_data = first_child.get_pc_data();
        assert_eq!(pc_data, "ali veli \"deneme yapmak\" = anlamında > bir deyim");
        let second_child = doc.get_xml_element(first_child.get_next_sibling().unwrap());
        assert_eq!(second_child.get_attribute_value("descr".to_string()).unwrap(), "perceiver, alien \"x3\" to whom?");
        assert_eq!(second_child.get_attribute_value("f".to_string()).unwrap(), "PAG");
        assert_eq!(second_child.get_attribute_value("n".to_string()).unwrap(), "2");
    }
}