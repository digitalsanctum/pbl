use crate::model::link::Link;
use serde::de::DeserializeOwned;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CiCd {
    name: String,
    file: String,
    link: Link,
}

impl CiCd {
    pub fn new(name: String,
               file: String,
               link: Link) -> CiCd {
        CiCd { name, file, link }
    }
}