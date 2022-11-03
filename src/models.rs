use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Label {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Repository {
    pub id: i64,
    pub name: String,
    pub html_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug)]
pub struct SortedIssues {
    pub priority_high_issues: Vec<Issue>,
    pub priority_medium_issues: Vec<Issue>,
    pub priority_low_issues: Vec<Issue>,
    pub priority_none_issues: Vec<Issue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SlackMessageBlock {
    #[serde(rename = "type")]
    pub(crate) block_type: String,
    pub(crate) text: Option<SlackMessageBlockText>,
    pub(crate) fields: Option<Vec<SlackMessageBlockField>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SlackMessageBlockText {
    #[serde(rename = "type")]
    pub(crate) text_type: String,
    pub(crate) text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SlackMessageBlockField {
    #[serde(rename = "type")]
    field_type: String,
    text: String,
}
