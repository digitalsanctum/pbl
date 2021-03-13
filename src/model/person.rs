#[derive(Clone, ramhorns::Content, Debug, Deserialize, Serialize)]
pub struct Person {
    name: String,
    email: Option<String>,
    company: Option<String>,
    location: Option<String>,
}

impl Person {}
