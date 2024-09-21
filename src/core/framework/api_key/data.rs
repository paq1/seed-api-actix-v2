#[derive(Clone, Debug)]
pub struct ApiKey {
    pub name: String,
    pub key: String,
}

impl ApiKey {
    pub fn new(name: &str, key: &str) -> Self {
        Self {
            name: name.to_string(),
            key: key.to_string()
        }
    }
}