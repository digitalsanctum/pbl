use crate::model::link::Link;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Team {
    name: String,
    email: Link,
    slack_channel: Link,
}

impl Team {
    pub fn new(name: String, email: Link, slack_channel: Link) -> Team {
        Team { name, email, slack_channel }
    }
}