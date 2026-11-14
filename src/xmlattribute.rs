pub struct XmlAttribute{
    name: String,
    value: String,
}

impl XmlAttribute{
    /// Constructor for xml attribute. Initializes the attribute.
    ///
    /// # Arguments
    ///
    /// * `name`: Name of the attribute
    ///
    /// returns: XmlAttribute
    ///
    pub fn new(name:String)->Self{
        Self{name, value: "".to_string() }
    }

    pub fn get_name(&self)->&str{
        &self.name
    }

    pub fn get_value(&self)->&str{
        &self.value
    }

    pub fn set_value(&mut self,value:String){
        self.value = value;
    }
}