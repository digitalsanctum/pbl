#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Link {
    label: String,
    url: String,
}

impl Link {
    pub fn new(label: String,
               url: String) -> Link {
        Link { label, url }
    }
}