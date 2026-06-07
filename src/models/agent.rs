use super::platform::AgentPlatform;
use super::skill::Scope;
use std::path::PathBuf;

/// A subagent definition
#[derive(Clone, Debug)]
pub struct Subagent {
    pub name: String,
    pub description: String,
    pub model: Option<String>,
    pub tools: Vec<String>,
    pub disallowed_tools: Vec<String>,
    pub mcp_servers: Vec<String>,
    pub permission_mode: Option<String>,
    pub body: String,
    pub path: PathBuf,
    pub platform: AgentPlatform,
    pub scope: Scope,
    pub format: AgentDefFormat,
}

#[derive(Clone, Debug, PartialEq)]
pub enum AgentDefFormat {
    MarkdownFrontmatter,
    Toml,
    InlineJson,
}
