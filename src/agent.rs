// # Agent Module

use anyhow::Result;
use rig::client::{CompletionClient, ProviderClient};
use rig::completion::Prompt;
use rig::providers::ollama;
use tracing::{debug, info};

use crate::config::Config;
use crate::tools::WebSearchTool;

// SYSTEM PROMPT
/// The system prompt defines the agent's personality and behavior.
const RESEARCH_SYSTEM_PROMPT: &str = r#"
You are a helpful AI research assistant. Your task is to research topics and provide summaries.

### CRITICAL TOOL USE RULE:
When calling the 'web_search' tool, the 'query' argument MUST be a plain text string ONLY.
- WRONG: web_search({"query": "..."})
- WRONG: web_search({"title": "...", "type": "string"})
- CORRECT: web_search("best solidity voting contract")

IMPORTANT INSTRUCTIONS:
1. Use the web_search tool ONCE to find relevant information
2. After getting search results, IMMEDIATELY synthesize them into a summary
3. DO NOT make multiple search requests - one search is sufficient
4. If the first search returns no results, try ONE simpler query, then summarize

When responding after a search:
- **Overview**: Brief introduction to the topic
- **Key Sources Found**: List the URLs from the search
- **Summary**: Synthesize what these sources likely cover based on their titles/domains
- **Next Steps**: Suggest what the user might explore

Always provide a response after seeing search results. Never keep searching indefinitely.
"#;

// RESEARCH AGENT STRUCT

/// We store a Config by value (owned). This means ResearchAgent owns
/// its configuration and will clean it up when dropped.
pub struct ResearchAgent {
    /// Configuration for the agent
    config: Config,

    /// The web search tool
    search_tool: WebSearchTool,
}

impl ResearchAgent {
    /// Create a new ResearchAgent with the given configuration.

    /// Rust doesn't have constructors like OOP languages.
    /// Instead, we use associated functions (usually named `new`).
    pub fn new(config: Config) -> Self {
        let search_tool = WebSearchTool::new(config.max_search_results);

        Self {
            config,
            search_tool,
        }
    }

    /// Research a topic and return a comprehensive summary.

    /// `&self` means we borrow the ResearchAgent immutably.
    /// `&str` for the query borrows the string data without copying.
    pub async fn research(&self, query: &str) -> Result<String> {
        info!(query = %query, "Starting research task");

        let q = query.to_lowercase();
        std::env::set_var("OLLAMA_API_BASE_URL", &self.config.ollama_host);
        if q == "hi" || q == "hello" {
            return Ok("Hello! How can I help you today?".into());
        }

        if q.contains('*') || q.contains('+') || q.contains('/') || q.contains('-') {
            info!("Math query detected, using simple agent without tools");
            let ollama_client = ollama::Client::from_env();
            let simple_agent = ollama_client
                .agent(&self.config.model)
                .preamble("You are a direct calculator. Provide only the numeric result.")
                .build();
            return Ok(simple_agent.prompt(query).await?);
        }

        // Step 1: Create the Ollama client using the builder pattern

        std::env::set_var("OLLAMA_API_BASE_URL", &self.config.ollama_host);

        let ollama_client = ollama::Client::from_env();

        debug!(
            host = %self.config.ollama_host,
            model = %self.config.model,
            "Connected to Ollama"
        );

        // Step 2: Build the agent with tools
        //
        // Rig's agent builder lets us:
        // - Set the model
        // - Add a system prompt (preamble)
        // - Register tools the agent can use
        let agent = ollama_client
            .agent(&self.config.model)
            .preamble(RESEARCH_SYSTEM_PROMPT)
            .tool(self.search_tool.clone())
            .build();

        info!("Agent configured, executing research query");

        // Step 3: Execute the research query
        let enhanced_query = format!(
            "Research the following topic thoroughly. Use the web_search tool to find \
             current information, then provide a comprehensive summary with sources:\n\n{}",
            query
        );

        let response = agent
            .prompt(&enhanced_query)
            .multi_turn(5) // Allow up to 5 iterations of tool calls
            .await
            .map_err(|e| anyhow::anyhow!("Agent execution failed: {}", e))?;

        info!("Research completed successfully");

        Ok(response)
    }

    /// Perform a quick search without full agent reasoning.
    pub async fn quick_search(&self, query: &str) -> Result<String> {
        info!(query = %query, "Performing quick search");

        let results = self
            .search_tool
            .search(query)
            .await
            .map_err(|e| anyhow::anyhow!("Search failed: {}", e))?;

        if results.is_empty() {
            return Ok(format!("No results found for: {}", query));
        }

        let formatted: String = results
            .iter()
            .enumerate()
            .map(|(i, r)| {
                format!(
                    "{}. **{}**\n   {}\n   URL: {}\n",
                    i + 1,
                    r.title,
                    r.snippet,
                    r.url
                )
            })
            .collect::<Vec<_>>()
            .join("\n");

        Ok(format!("## Search Results\n\n{}", formatted))
    }
}

// UNIT TESTS
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_creation() {
        let config = Config::default();
        let agent = ResearchAgent::new(config);

        assert_eq!(agent.config.model, "llama3.2");
    }

    #[test]
    fn test_system_prompt_not_empty() {
        assert!(!RESEARCH_SYSTEM_PROMPT.is_empty());
        assert!(RESEARCH_SYSTEM_PROMPT.contains("research"));
    }
}
