use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Label {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Issue {
    pub id: i64,
    pub title: String,
    pub url: String,
    pub state: String,
    pub body: Option<String>,
    pub labels: Option<Vec<Label>>,
}
