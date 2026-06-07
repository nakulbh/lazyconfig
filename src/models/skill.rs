use super::platform::AgentPlatform;
use std::collections::HashMap;
use std::path::PathBuf;

/// A skill following the Agent Skills open standard
#[derive(Clone, Debug)]
pub struct Skill {
    /// From frontmatter: 1-64 chars, [a-z0-9-]+
    pub name: String,
    /// From frontmatter: 1-1024 chars
    pub description: String,
    /// Optional license
    pub license: Option<String>,
    /// Optional compatibility info
    pub compatibility: Option<String>,
    /// Optional metadata key-value pairs
    pub metadata: HashMap<String, String>,
    /// Optional allowed tools (experimental)
    pub allowed_tools: Option<String>,
    /// Markdown body after frontmatter
    pub body: String,
    /// Absolute path to SKILL.md
    pub path: PathBuf,
    /// Which agent platform this skill belongs to
    pub platform: AgentPlatform,
    /// Global or project-local
    pub scope: Scope,
}

#[derive(Clone, Debug)]
pub enum Scope {
    Global,
    Project(PathBuf),
}
