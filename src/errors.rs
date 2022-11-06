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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_issue_error_1() {
        let error = GetIssueError {
            message: "test error".to_string(),
        };
        assert_eq!(error.message, "test error");
    }
}
