//! Per-agent launch configuration.
//!
//! ACP normalizes the protocol, so the only agent-specific knowledge the harness
//! needs is *how to start each agent as an ACP subprocess*: the program, its
//! arguments, and any environment. Everything after launch flows through the one
//! [`super::connection::AcpConnection`]. The v1 slice ships the Claude Code
//! launcher; Codex and opencode are scaffolded here so adding them is a data
//! change, not new plumbing.

use serde::{Deserialize, Serialize};

/// The agents the harness can launch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AgentKind {
    ClaudeCode,
    Codex,
    Opencode,
}

/// How to spawn a given agent as an ACP subprocess.
#[derive(Debug, Clone)]
pub struct LaunchSpec {
    /// Executable to run.
    pub program: String,
    /// Arguments passed to the executable.
    pub args: Vec<String>,
}

impl AgentKind {
    /// Resolve the launch spec for this agent.
    ///
    /// Claude Code speaks ACP via the `@zed-industries/claude-code-acp` adapter,
    /// run through `npx` so no global install is required. Codex and opencode are
    /// placeholders until their launchers are validated in v1.5.
    pub fn launch_spec(self) -> LaunchSpec {
        match self {
            AgentKind::ClaudeCode => LaunchSpec {
                program: "npx".into(),
                args: vec!["-y".into(), "@zed-industries/claude-code-acp".into()],
            },
            AgentKind::Codex => LaunchSpec {
                program: "codex".into(),
                args: vec!["acp".into()],
            },
            AgentKind::Opencode => LaunchSpec {
                program: "opencode".into(),
                args: vec!["acp".into()],
            },
        }
    }

    /// Human-readable label.
    pub fn label(self) -> &'static str {
        match self {
            AgentKind::ClaudeCode => "Claude Code",
            AgentKind::Codex => "Codex CLI",
            AgentKind::Opencode => "opencode",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn claude_code_launches_via_npx_adapter() {
        let spec = AgentKind::ClaudeCode.launch_spec();
        assert_eq!(spec.program, "npx");
        assert!(spec
            .args
            .iter()
            .any(|a| a.contains("claude-code-acp")));
    }

    #[test]
    fn agent_kind_round_trips_through_serde() {
        let json = serde_json::to_string(&AgentKind::ClaudeCode).unwrap();
        assert_eq!(json, "\"claude_code\"");
        let back: AgentKind = serde_json::from_str("\"opencode\"").unwrap();
        assert_eq!(back, AgentKind::Opencode);
    }
}
