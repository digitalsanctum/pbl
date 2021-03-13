#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IssueTracker {
    name: String,
    base_url: String,
    create_issue_url: String,
    existing_issues_url: String,
}

impl IssueTracker {
    pub fn new(name: String,
               base_url: String,
               create_issue_url: String,
               existing_issues_url: String, ) -> IssueTracker {
        IssueTracker { name, base_url, create_issue_url, existing_issues_url }
    }
}