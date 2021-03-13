#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MergeRequirements {
    min_approvals: u8,
    min_maintainer_approvals: u8,
}

impl MergeRequirements {
    pub fn new(min_approvals: u8,
               min_maintainer_approvals: u8, ) -> MergeRequirements {
        MergeRequirements { min_approvals, min_maintainer_approvals }
    }
}