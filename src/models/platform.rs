use std::path::PathBuf;

/// Supported AI agent platforms
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum AgentPlatform {
    ClaudeCode,
    OpenCode,
    CodexCli,
    GitHubCopilot,
    // Phase 6+:
    // Cursor,
    // Windsurf,
    // GeminiCli,
    // Cline,
    // Amp,
    // Antigravity,
    // ClawdBot,
    // Droid,
    // Goose,
    // Kilo,
    // KiroCli,
    // NousResearch,
    // RooCode,
    // Trae,
    // VSCode,
    // Zed,
}

impl AgentPlatform {
    pub fn name(&self) -> &'static str {
        match self {
            AgentPlatform::ClaudeCode => "Claude Code",
            AgentPlatform::OpenCode => "OpenCode",
            AgentPlatform::CodexCli => "Codex CLI",
            AgentPlatform::GitHubCopilot => "GitHub Copilot",
        }
    }

    pub fn short_name(&self) -> &'static str {
        match self {
            AgentPlatform::ClaudeCode => "claude",
            AgentPlatform::OpenCode => "opencode",
            AgentPlatform::CodexCli => "codex",
            AgentPlatform::GitHubCopilot => "copilot",
        }
    }

    pub fn global_skills_path(&self) -> Option<PathBuf> {
        match self {
            AgentPlatform::ClaudeCode => dirs::home_dir().map(|h| h.join(".claude/skills")),
            AgentPlatform::OpenCode => dirs::home_dir().map(|h| h.join(".config/opencode/skills")),
            AgentPlatform::CodexCli => dirs::home_dir().map(|h| h.join(".agents/skills")),
            AgentPlatform::GitHubCopilot => dirs::home_dir().map(|h| h.join(".copilot/skills")),
        }
    }

    pub fn project_skills_path(&self, project_root: &PathBuf) -> Option<PathBuf> {
        match self {
            AgentPlatform::ClaudeCode => Some(project_root.join(".claude/skills")),
            AgentPlatform::OpenCode => Some(project_root.join(".opencode/skills")),
            AgentPlatform::CodexCli => Some(project_root.join(".agents/skills")),
            AgentPlatform::GitHubCopilot => Some(project_root.join(".github/skills")),
        }
    }

    pub fn global_agents_path(&self) -> Option<PathBuf> {
        match self {
            AgentPlatform::ClaudeCode => dirs::home_dir().map(|h| h.join(".claude/agents")),
            AgentPlatform::OpenCode => dirs::home_dir().map(|h| h.join(".config/opencode/agents")),
            AgentPlatform::CodexCli => dirs::home_dir().map(|h| h.join(".codex/agents")),
            AgentPlatform::GitHubCopilot => None, // No global agents for Copilot
        }
    }

    pub fn project_agents_path(&self, project_root: &PathBuf) -> Option<PathBuf> {
        match self {
            AgentPlatform::ClaudeCode => Some(project_root.join(".claude/agents")),
            AgentPlatform::OpenCode => Some(project_root.join(".opencode/agents")),
            AgentPlatform::CodexCli => Some(project_root.join(".codex/agents")),
            AgentPlatform::GitHubCopilot => Some(project_root.join(".github/agents")),
        }
    }

    pub fn all() -> Vec<AgentPlatform> {
        vec![
            AgentPlatform::ClaudeCode,
            AgentPlatform::OpenCode,
            AgentPlatform::CodexCli,
            AgentPlatform::GitHubCopilot,
        ]
    }
}
