use std::iter::Map;

pub struct FileNames {
  pub messages: &'static str,
  pub sections: &'static str,
  pub snippets: &'static str,
}

pub struct RpMessage {
  labels: Vec<String>,
  sections: Map<String, Vec<Section>>,
}


pub struct Section {
  name: String,
  snippets: Vec<Snippet>,
}

pub struct Snippet {
  name: String,
  labels: Vec<String>,
  tokens: Map<String, String>,
  body: String,
}