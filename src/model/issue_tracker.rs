use crate::model::link::Link;

#[derive(Clone, ramhorns::Content, Debug, Deserialize, Serialize)]
pub struct IssueTracker {
    tool: Link,
    create_issue: Link,
    existing_issues: Link,
}

impl IssueTracker {}
