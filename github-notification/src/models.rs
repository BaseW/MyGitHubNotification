use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Label {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Repository {
    pub id: i64,
    pub name: String,
    pub html_url: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Issue {
    pub id: i64,
    pub title: String,
    pub html_url: String,
    pub state: String,
    pub body: Option<String>,
    pub labels: Option<Vec<Label>>,
    pub repository: Repository,
    pub label_string: Option<String>,
}

#[derive(Debug, Default)]
pub struct SortedIssues {
    pub priority_high_issues: Vec<Issue>,
    pub priority_medium_issues: Vec<Issue>,
    pub priority_low_issues: Vec<Issue>,
    pub priority_none_issues: Vec<Issue>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_label_1() {
        let label = Label {
            id: 1,
            name: "test".to_string(),
        };
        assert_eq!(label.id, 1);
        assert_eq!(label.name, "test");
    }

    #[test]
    fn test_repository_1() {
        let repo = Repository {
            id: 1,
            name: "test".to_string(),
            html_url: "test".to_string(),
        };
        assert_eq!(repo.id, 1);
        assert_eq!(repo.name, "test");
        assert_eq!(repo.html_url, "test");
    }

    #[test]
    fn test_issue_1() {
        let issue = Issue {
            id: 1,
            title: "test".to_string(),
            html_url: "test".to_string(),
            state: "test".to_string(),
            body: Some("test".to_string()),
            labels: None,
            repository: Repository {
                id: 1,
                name: "test".to_string(),
                html_url: "test".to_string(),
            },
            label_string: None,
        };
        assert_eq!(issue.id, 1);
        assert_eq!(issue.title, "test");
        assert_eq!(issue.html_url, "test");
        assert_eq!(issue.state, "test");
        assert_eq!(issue.body, Some("test".to_string()));
        assert_eq!(issue.labels, None);
        assert_eq!(issue.repository.id, 1);
        assert_eq!(issue.repository.name, "test");
        assert_eq!(issue.repository.html_url, "test");
        assert_eq!(issue.label_string, None);
    }

    #[test]
    fn test_sorted_issues_1() {
        let sorted_issues: SortedIssues = Default::default();
        assert_eq!(sorted_issues.priority_high_issues.len(), 0);
        assert_eq!(sorted_issues.priority_medium_issues.len(), 0);
        assert_eq!(sorted_issues.priority_low_issues.len(), 0);
        assert_eq!(sorted_issues.priority_none_issues.len(), 0);
    }
}
