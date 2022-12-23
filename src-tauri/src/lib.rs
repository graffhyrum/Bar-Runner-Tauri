use std::iter::Map;

pub struct FileNames {
    pub messages: &'static str,
    pub sections: &'static str,
    pub snippets: &'static str,
}

pub struct MsgFolder {
    id: String,
    name: String,
    ctime: String,
}

pub struct Message {
    //ie some_drink
    id: String,
    // server ID
    name: String,
    // "Screwdriver"
    labels: Vec<String>,
    // "Asmo,Drink,Highball"
    sections: Vec<String>, // Vec of section IDs
}

pub struct Section {
    id: String,
    name: String,
    variants: Vec<Variant>,
}

pub struct Variant {
    id: String,
    name: String,
    labels: Vec<String>,
    body: Option<String>,
}

trait GetId {
    fn get_id(&self) -> String;
}

impl GetId for Message {
    fn get_id(&self) -> String { self.id.to_string() }
}

impl GetId for Section {
    fn get_id(&self) -> String {
        self.id.to_string()
    }
}

impl GetId for Variant {
    fn get_id(&self) -> String {
        self.id.to_string()
    }
}
