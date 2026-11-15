use crate::xmlattribute::XmlAttribute;

pub struct XmlElement{
    name: String,
    pc_data: String,
    attributes: Vec<XmlAttribute>,
    parent: Option<usize>,
    first_child: Option<usize>,
    next_sibling: Option<usize>
}

impl XmlElement{

    /// Constructor for xml element. Allocates memory and initializes an element.
    ///
    /// # Arguments
    ///
    /// * `name`: Name of the element
    /// * `parent`: Parent of the Xml Element
    ///
    /// returns: XmlElement
    ///
    pub fn new(name:String, parent: Option<usize>)->Self{
        Self{name, pc_data:"".to_string(), attributes:Vec::new(), parent: parent, first_child: None, next_sibling: None}
    }

    pub fn get_name(&self)->&str{
        &self.name
    }

    pub fn get_pc_data(&self)->&str{
        &self.pc_data
    }

    pub fn set_pc_data(&mut self, data:String){
        self.pc_data = data
    }

    pub fn get_first_child(&self)->Option<usize>{
        self.first_child
    }

    pub fn set_first_child(&mut self, first_child:usize){
        self.first_child = Some(first_child);
    }

    pub fn get_next_sibling(&self)->Option<usize>{
        self.next_sibling
    }

    pub fn set_next_sibling(&mut self, sibling: usize){
        self.next_sibling = Some(sibling);
    }

    pub fn get_parent(&self)->Option<usize>{
        self.parent
    }

    /// Finds the attribute with the given name of an Xml element
    ///
    /// # Arguments
    ///
    /// * `attribute_name`: Name of the attribute
    ///
    /// returns: Option<&String> If the Xml element has such an attribute returns its value, otherwise it returns None
    ///
    pub fn get_attribute_value(&self, attribute_name: String) -> Option<&str> {
        for attribute in &self.attributes{
            if *attribute.get_name() == attribute_name {
                return Some(attribute.get_value())
            }
        }
        None
    }

    /// Sets the value of an attribute to a given value
    ///
    /// # Arguments
    ///
    /// * `attribute_name`: Name of the attribute
    /// * `value`: New attribute value
    ///
    pub fn set_attribute_value(&mut self, attribute_name: &String, value: String){
        for attribute in &mut self.attributes{
            if attribute.get_name() == attribute_name {
                attribute.set_value(value.clone());
            }
        }
    }

    pub fn get_attribute(&self, index: usize)->&XmlAttribute{
        &(self.attributes[index])
    }

    pub fn add_attribute(&mut self, attribute:XmlAttribute){
        self.attributes.push(attribute);
    }

    pub fn attribute_size(&self)->usize{
        self.attributes.len()
    }

    pub fn has_attributes(&self)->bool{
        !self.attributes.is_empty()
    }
}