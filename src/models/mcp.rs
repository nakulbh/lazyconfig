use super::platform::AgentPlatform;
use super::skill::Scope;
use std::path::PathBuf;

/// MCP Server configuration
#[derive(Clone, Debug)]
pub struct MCPServer {
    pub name: String,
    pub transport: MCPTransport,
    pub enabled: bool,
    pub platform: AgentPlatform,
    pub scope: Scope,
    pub source_file: PathBuf,
}

#[derive(Clone, Debug)]
pub enum MCPTransport {
    Stdio {
        command: String,
        args: Vec<String>,
        env: Vec<(String, String)>,
    },
    HTTP {
        url: String,
        headers: Vec<(String, String)>,
    },
}
