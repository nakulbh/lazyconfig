use std::path::PathBuf;

/// A rule or instruction file
#[derive(Clone, Debug)]
pub struct Rule {
    pub name: String,
    pub description: Option<String>,
    pub paths: Vec<String>, // glob patterns for path-scoped rules
    pub body: String,
    pub path: PathBuf,
}

/// A hook configuration
#[derive(Clone, Debug)]
pub struct Hook {
    pub name: String,
    pub event: String, // PreToolUse, PostToolUse, etc.
    pub matcher: Option<String>,
    pub command: String,
    pub path: PathBuf,
}
