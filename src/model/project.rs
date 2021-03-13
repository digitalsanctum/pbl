use crate::model::issue_tracker::IssueTracker;
use crate::model::ci_cd::CiCd;
use crate::model::person::Person;
use crate::model::team::Team;
use crate::model::merge_requirements::MergeRequirements;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Project {
    name: String,
    repo_name: String,
    change_log: String,
    issue_tracker: IssueTracker,
    merge_requirements: MergeRequirements,
    ci_cd: CiCd,
    contributors: Vec<Person>,
    maintainers: Vec<Person>,
    team: Team
}

impl Project {
    pub fn new(name: String,
               repo_name: String,
               change_log: String,
               issue_tracker: IssueTracker,
               merge_requirements: MergeRequirements,
               ci_cd: CiCd,
               contributors: Vec<Person>,
               maintainers: Vec<Person>,
               team: Team) -> Project {
        Project {
            name,
            repo_name,
            change_log,
            issue_tracker,
            merge_requirements,
            ci_cd,
            contributors,
            maintainers,
            team
        }
    }
}