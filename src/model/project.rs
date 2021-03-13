use crate::model::ci_cd::CiCd;
use crate::model::issue_tracker::IssueTracker;
use crate::model::link::Link;
use crate::model::merge_requirements::MergeRequirements;
use crate::model::person::Person;

#[derive(Clone, ramhorns::Content, Debug, Deserialize, Serialize)]
pub struct Project {
    name: String,
    description: Option<String>,
    repo: Link,
    change_log: Link,
    issue_tracker: Option<IssueTracker>,
    merge_requirements: Option<MergeRequirements>,
    ci_cd: Option<CiCd>,
    contributors: Option<Vec<Person>>,
    maintainers: Option<Vec<Person>>,
    communications: Option<Vec<Link>>,
}

impl Project {}
