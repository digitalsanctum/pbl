use crate::model::link::Link;

#[derive(Clone, ramhorns::Content, Debug, Deserialize, Serialize)]
pub struct CiCd {
    tool: Link,
    files: Vec<Link>,
}

impl CiCd {}
