#[derive(Clone, ramhorns::Content, Debug, Deserialize, Serialize)]
pub struct MergeRequirements {
    min_approvals: u8,
    min_maintainer_approvals: u8,
}

impl MergeRequirements {}
