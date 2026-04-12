#[derive(Debug, Clone)]
pub enum CommandType {
    Help,
    Status,
    Clear,
    Agents,
    Tools,
    Query(String),
    Unknown(String),
}

impl CommandType {
    pub fn parse(input: &str) -> Self {
        let parts: Vec<&str> = input.split_whitespace().collect();

        match parts.first() {
            Some(&"/help") => CommandType::Help,
            Some(&"/status") => CommandType::Status,
            Some(&"/clear") => CommandType::Clear,
            Some(&"/agents") => CommandType::Agents,
            Some(&"/tools") => CommandType::Tools,
            Some(cmd) if cmd.starts_with('/') => CommandType::Unknown(cmd.to_string()),
            Some(query) => CommandType::Query(query.to_string()),
            None => CommandType::Query(input.to_string()),
        }
    }

    pub fn execute(&self) -> String {
        match self {
            CommandType::Help => Self::help(),
            CommandType::Status => Self::status(),
            CommandType::Clear => "CLEAR_SCREEN".to_string(),
            CommandType::Agents => Self::agents(),
            CommandType::Tools => Self::tools(),
            CommandType::Unknown(cmd) => format!("Unknown command: {}", cmd),
            CommandType::Query(query) => Self::query(query),
        }
    }

    fn help() -> String {
        r#"Available Commands:
  /help          - Show this help message
  /status        - Show agent status
  /clear         - Clear the screen
  /agents        - List available agents
  /tools         - List available tools
  /exit          - Exit the application
  <query>        - Send a query to the current agent

Navigation:
  Enter          - Send command
  Esc            - Quit"#
            .to_string()
    }

    fn status() -> String {
        "Agent Status: Active\nMode: Standard\nReady to process commands.".to_string()
    }

    fn agents() -> String {
        r#"Available Agents:
  * code-assistant    - General purpose coding assistant
    debugger         - Debugging specialist
    reviewer         - Code review specialist
    architect        - System design and architecture advisor"#
            .to_string()
    }

    fn tools() -> String {
        r#"Available Tools:
  * file-reader      - Read files and directories
    file-writer      - Create and modify files
    shell            - Execute shell commands
    search           - Search codebase
    git              - Git operations"#
            .to_string()
    }

    fn query(query: &str) -> String {
        format!(
            "Processing query: \"{}\"\n\n\
             [NULCODE Agent Response]\n\
             This is a simulated response from the NULCODE Agent.\n\
             In a real implementation, this would connect to an AI model\n\
             to provide intelligent responses to your coding questions.\n\n\
             Query length: {} characters",
            query,
            query.len()
        )
    }
}
