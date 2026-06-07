use crate::models::platform::AgentPlatform;
use crate::models::agent::{Subagent, AgentDefFormat};
use crate::models::skill::Scope;
use crate::models::mcp::{MCPServer, MCPTransport};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub struct Scanner;

impl Scanner {
    pub fn new() -> Self {
        Self
    }

    pub fn scan_all_skills(&self) -> Vec<crate::models::skill::Skill> {
        let mut skills = Vec::new();
        for platform in AgentPlatform::all() {
            if let Some(global_path) = platform.global_skills_path() {
                skills.extend(self.scan_skills_in_dir(&global_path, platform, Scope::Global));
            }
            for project in self.detect_projects() {
                if let Some(project_path) = platform.project_skills_path(&PathBuf::from(&project)) {
                    skills.extend(self.scan_skills_in_dir(
                        &project_path,
                        platform,
                        Scope::Project(PathBuf::from(&project)),
                    ));
                }
            }
        }
        skills
    }

    pub fn scan_all_agents(&self) -> Vec<Subagent> {
        let mut agents = Vec::new();
        for platform in AgentPlatform::all() {
            if let Some(global_path) = platform.global_agents_path() {
                agents.extend(self.scan_agents_in_dir(&global_path, platform, Scope::Global));
            }
            for project in self.detect_projects() {
                if let Some(project_path) = platform.project_agents_path(&PathBuf::from(&project)) {
                    agents.extend(self.scan_agents_in_dir(
                        &project_path,
                        platform,
                        Scope::Project(PathBuf::from(&project)),
                    ));
                }
            }
            if platform == AgentPlatform::OpenCode {
                agents.extend(self.scan_opencode_json_agents(platform));
            }
        }
        agents
    }

    pub fn scan_all_mcp_servers(&self) -> Vec<MCPServer> {
        let mut servers = Vec::new();
        for platform in AgentPlatform::all() {
            servers.extend(self.scan_mcp_for_platform(platform));
        }
        servers
    }

    fn scan_skills_in_dir(&self, dir: &PathBuf, platform: AgentPlatform, scope: Scope) -> Vec<crate::models::skill::Skill> {
        let mut skills = Vec::new();
        if !dir.exists() { return skills; }
        for entry in WalkDir::new(dir).max_depth(2).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.file_name().map(|f| f == "SKILL.md").unwrap_or(false) {
                if let Some(skill) = self.parse_skill_file(path, platform, scope.clone()) {
                    skills.push(skill);
                }
            }
        }
        skills
    }

    fn scan_agents_in_dir(&self, dir: &PathBuf, platform: AgentPlatform, scope: Scope) -> Vec<Subagent> {
        let mut agents = Vec::new();
        if !dir.exists() { return agents; }
        for entry in WalkDir::new(dir).max_depth(2).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            let ext = path.extension().and_then(|e| e.to_str());
            let fname = path.file_name().and_then(|f| f.to_str()).unwrap_or("");
            if ext == Some("md") && fname.ends_with(".md") && fname != "SKILL.md" {
                if let Some(agent) = self.parse_agent_md(path, platform, scope.clone()) {
                    agents.push(agent);
                }
            } else if ext == Some("toml") {
                if let Some(agent) = self.parse_agent_toml(path, platform, scope.clone()) {
                    agents.push(agent);
                }
            }
        }
        agents
    }

    fn scan_opencode_json_agents(&self, platform: AgentPlatform) -> Vec<Subagent> {
        let mut agents = Vec::new();
        let config_paths = vec![
            dirs::home_dir().map(|h| h.join(".config/opencode/opencode.json")),
        ];
        for config_path in config_paths.into_iter().flatten() {
            if !config_path.exists() { continue; }
            if let Ok(content) = std::fs::read_to_string(&config_path) {
                if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
                    if let Some(agent_obj) = json.get("agent").and_then(|a| a.as_object()) {
                        for (name, config) in agent_obj {
                            let description = config.get("description")
                                .and_then(|d| d.as_str())
                                .unwrap_or("No description")
                                .to_string();
                            let model = config.get("model").and_then(|m| m.as_str()).map(|s| s.to_string());
                            let mode = config.get("mode").and_then(|m| m.as_str()).unwrap_or("all");
                            agents.push(Subagent {
                                name: name.clone(),
                                description,
                                model,
                                tools: Vec::new(),
                                disallowed_tools: Vec::new(),
                                mcp_servers: Vec::new(),
                                permission_mode: Some(mode.to_string()),
                                body: String::new(),
                                path: config_path.clone(),
                                platform,
                                scope: Scope::Global,
                                format: AgentDefFormat::InlineJson,
                            });
                        }
                    }
                }
            }
        }
        agents
    }

    fn scan_mcp_for_platform(&self, platform: AgentPlatform) -> Vec<MCPServer> {
        let mut servers = Vec::new();
        let config_paths: Vec<PathBuf> = match platform {
            AgentPlatform::ClaudeCode => {
                let mut paths = Vec::new();
                if let Some(h) = dirs::home_dir() {
                    paths.push(h.join(".claude/.mcp.json"));
                }
                paths
            }
            AgentPlatform::OpenCode => {
                let mut paths = Vec::new();
                if let Some(h) = dirs::home_dir() {
                    paths.push(h.join(".config/opencode/opencode.json"));
                }
                paths
            }
            AgentPlatform::CodexCli => {
                let mut paths = Vec::new();
                if let Some(h) = dirs::home_dir() {
                    paths.push(h.join(".codex/config.toml"));
                }
                paths
            }
            AgentPlatform::GitHubCopilot => {
                Vec::new()
            }
        };

        for config_path in config_paths {
            if !config_path.exists() { continue; }
            let content = match std::fs::read_to_string(&config_path) {
                Ok(c) => c,
                Err(_) => continue,
            };

            match config_path.extension().and_then(|e| e.to_str()) {
                Some("json") => {
                    if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
                        let mcp_obj = if platform == AgentPlatform::OpenCode {
                            json.get("mcp").and_then(|m| m.as_object())
                        } else {
                            json.as_object()
                        };
                        if let Some(mcp_map) = mcp_obj {
                            for (name, config) in mcp_map {
                                let transport = if let Some(url) = config.get("url").and_then(|u| u.as_str()) {
                                    let mut headers = Vec::new();
                                    if let Some(h) = config.get("headers").and_then(|h| h.as_object()) {
                                        for (k, v) in h {
                                            if let Some(val) = v.as_str() {
                                                headers.push((k.clone(), val.to_string()));
                                            }
                                        }
                                    }
                                    MCPTransport::HTTP { url: url.to_string(), headers }
                                } else {
                                    let command = config.get("command")
                                        .and_then(|c| {
                                            if let Some(arr) = c.as_array() {
                                                arr.first().and_then(|f| f.as_str()).map(|s| s.to_string())
                                            } else {
                                                c.as_str().map(|s| s.to_string())
                                            }
                                        })
                                        .unwrap_or_default();
                                    let args = config.get("args")
                                        .and_then(|a| a.as_array())
                                        .map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
                                        .unwrap_or_default();
                                    MCPTransport::Stdio { command, args, env: Vec::new() }
                                };
                                let enabled = config.get("enabled").and_then(|e| e.as_bool()).unwrap_or(true);
                                servers.push(MCPServer {
                                    name: name.clone(),
                                    transport,
                                    enabled,
                                    platform,
                                    scope: Scope::Global,
                                    source_file: config_path.clone(),
                                });
                            }
                        }
                    }
                }
                Some("toml") => {
                    if let Ok(toml_val) = content.parse::<toml::Table>() {
                        if let Some(mcp_servers) = toml_val.get("mcp_servers").and_then(|m| m.as_table()) {
                            for (name, config) in mcp_servers {
                                let command = config.get("command").and_then(|c| c.as_str()).unwrap_or("").to_string();
                                let args = config.get("args")
                                    .and_then(|a| a.as_array())
                                    .map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect())
                                    .unwrap_or_default();
                                let enabled = config.get("enabled").and_then(|e| e.as_bool()).unwrap_or(true);
                                servers.push(MCPServer {
                                    name: name.clone(),
                                    transport: MCPTransport::Stdio { command, args, env: Vec::new() },
                                    enabled,
                                    platform,
                                    scope: Scope::Global,
                                    source_file: config_path.clone(),
                                });
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        servers
    }

    fn parse_skill_file(&self, path: &Path, platform: AgentPlatform, scope: Scope) -> Option<crate::models::skill::Skill> {
        let content = std::fs::read_to_string(path).ok()?;
        let (frontmatter, body) = Self::extract_frontmatter(&content)?;
        let name = frontmatter.get("name")?.clone();
        let description = frontmatter.get("description")?.clone();
        Some(crate::models::skill::Skill {
            name,
            description,
            license: frontmatter.get("license").cloned(),
            compatibility: frontmatter.get("compatibility").cloned(),
            metadata: HashMap::new(),
            allowed_tools: frontmatter.get("allowed-tools").cloned(),
            body,
            path: path.to_path_buf(),
            platform,
            scope,
        })
    }

    fn parse_agent_md(&self, path: &Path, platform: AgentPlatform, scope: Scope) -> Option<Subagent> {
        let content = std::fs::read_to_string(path).ok()?;
        let (frontmatter, body) = Self::extract_frontmatter(&content)?;
        let name = path.file_stem()?.to_string_lossy().to_string();
        let description = frontmatter.get("description").cloned().unwrap_or_default();
        let model = frontmatter.get("model").cloned();
        let permission_mode = frontmatter.get("permissionMode").cloned()
            .or_else(|| frontmatter.get("permission_mode").cloned());
        let tools = frontmatter.get("tools")
            .map(|t| t.split(',').map(|s| s.trim().to_string()).collect())
            .unwrap_or_default();
        Some(Subagent {
            name,
            description,
            model,
            tools,
            disallowed_tools: Vec::new(),
            mcp_servers: Vec::new(),
            permission_mode,
            body,
            path: path.to_path_buf(),
            platform,
            scope,
            format: AgentDefFormat::MarkdownFrontmatter,
        })
    }

    fn parse_agent_toml(&self, path: &Path, platform: AgentPlatform, scope: Scope) -> Option<Subagent> {
        let content = std::fs::read_to_string(path).ok()?;
        let toml_val: toml::Table = content.parse().ok()?;
        let name = toml_val.get("name")?.as_str()?.to_string();
        let description = toml_val.get("description").and_then(|d| d.as_str()).unwrap_or("").to_string();
        let model = toml_val.get("model").and_then(|m| m.as_str()).map(|s| s.to_string());
        let body = toml_val.get("developer_instructions").and_then(|d| d.as_str()).unwrap_or("").to_string();
        Some(Subagent {
            name,
            description,
            model,
            tools: Vec::new(),
            disallowed_tools: Vec::new(),
            mcp_servers: Vec::new(),
            permission_mode: None,
            body,
            path: path.to_path_buf(),
            platform,
            scope,
            format: AgentDefFormat::Toml,
        })
    }

    fn extract_frontmatter(content: &str) -> Option<(HashMap<String, String>, String)> {
        let lines: Vec<&str> = content.lines().collect();
        if lines.is_empty() || lines[0].trim() != "---" { return None; }
        let mut frontmatter = HashMap::new();
        let mut body_start = 0;
        for (i, line) in lines.iter().enumerate().skip(1) {
            if line.trim() == "---" {
                body_start = i + 1;
                break;
            }
            if let Some((key, value)) = line.split_once(':') {
                frontmatter.insert(key.trim().to_string(), value.trim().to_string());
            }
        }
        let body = lines[body_start..].join("\n");
        Some((frontmatter, body))
    }

    fn detect_projects(&self) -> Vec<String> {
        let mut projects = Vec::new();
        let home = dirs::home_dir().unwrap_or_default();
        let search_dirs = vec![
            home.join("Developer"),
            home.join("projects"),
            home.join("code"),
            home.join("workspace"),
        ];
        for search_dir in search_dirs {
            if !search_dir.exists() { continue; }
            for entry in std::fs::read_dir(&search_dir).into_iter().flatten() {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_dir() && path.join(".git").exists() {
                        if path.join(".claude").exists()
                            || path.join(".opencode").exists()
                            || path.join(".agents").exists()
                            || path.join(".github").exists()
                        {
                            projects.push(path.to_string_lossy().to_string());
                        }
                    }
                }
            }
        }
        projects
    }
}
