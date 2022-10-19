use std::error;

#[derive(Debug, Clone)]
pub struct GetIssueError {
    pub message: String,
}

// implement for GetIssueError
impl std::fmt::Display for GetIssueError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl error::Error for GetIssueError {}
