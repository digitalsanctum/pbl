#[derive(Clone, ramhorns::Content, Debug, Default, Deserialize, Serialize)]
pub struct Link {
    prefix: Option<String>,
    label: String,
    url: String,
    suffix: Option<String>,
}

impl Link {}
