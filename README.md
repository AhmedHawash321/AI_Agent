# 🤖 Autonomous AI Research Agent
> **High-Performance, Local-First AI Agent built with Rust, Rig, and Ollama.**

[![Rust](https://img.shields.io/badge/language-Rust-orange.svg)](https://www.rust-lang.org/)
[![Framework](https://img.shields.io/badge/Framework-Rig-green.svg)](https://github.com/0xPlayground/rig)
[![Inference](https://img.shields.io/badge/Inference-Ollama-blue.svg)](https://ollama.ai/)
[![License](https://img.shields.io/badge/license-MIT-lightgrey.svg)](LICENSE)

---

## 🌟 Overview
This isn't just another LLM wrapper. It's a full **Agentic Workflow** that thinks, searches, and synthesizes.

* **🔍 Autonomous Research:** Custom scraper for real-time DuckDuckGo data.
* **🧠 Multi-Turn Reasoning:** Up to 5 autonomous iterations to refine findings.
* **⚡ Smart Routing:** Instant handling for math & greetings (zero-token waste).
* **🛡️ Local-First:** 100% privacy. Your data never leaves your machine.

---

## 🛠️ Tech Stack & Architecture

ConceptWhere UsedAsync/Await + Tokioagent.rs, tools.rs — fully async pipelineTrait ImplementationTool trait from Rig — enables agent tool useCustom Error Typesthiserror in tools.rs — typed SearchError enumBox<dyn Error>Dynamic error handling across module boundariesStruct + implResearchAgent, Config, WebSearchTool#[derive(...)]Debug, Clone, Serialize, Deserialize, ParserBuilder Patternollama_client.agent().preamble().tool().build()Result<T, E>Every fallible function returns Result? OperatorClean error propagation without boilerplateEnvironment Variablesstd::env::var() + dotenvy for configOwnership in Async&self borrows, .clone() for tool sharingPattern Matchingmatch, if let, unwrap_or_elseIterators.map(), .filter(), .collect(), .enumerate()

⚙️ Tech Stack
DependencyPurposerig-core 0.27LLM agent framework with tool supporttokioAsync runtimeollama (via rig)Local LLM inferencereqwestHTTP client for web searchclapCLI argument parsingserde / serde_jsonSerializationanyhowFlexible error handlingthiserrorTyped custom errorstracingStructured loggingdotenvy.env file loading

### 📁 Project Structure
- `src/main.rs`: Application entry point & CLI logic.
- `src/agent.rs`: Core Agent implementation & reasoning loops.
- `src/tools.rs`: Web search tools & specialized scrapers.
- `src/config.rs`: Environment & settings management.

---

## 🚀 Quick Start

### 1. Prerequisites
- **Install Rust:** `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- **Install Ollama:** Visit [ollama.ai](https://ollama.ai)
- **Prepare Models:**
  ```bash
  ollama pull llama3.2
  ollama serve

2 - Installation
```bash
# Clone the repository
git clone [https://github.com/AhmedHawash321/AI_Agent.git](https://github.com/AhmedHawash321/AI_Agent.git)

# Navigate to the project folder
cd AI_Agent

# Copy environment template
cp .env.example .env

# Build the project
cargo build --release
```
3. Usage Examples
```bash
   # Deep Research (Default)
cargo run -- "Latest trends in Rust WebAssembly 2026"

# Quick Search (No AI Synthesis)
cargo run -- --quick "Solidity security best practices"

# Custom Model & Verbose Logging
cargo run -- --model deepseek-v3.2 --verbose "Quantum computing status"
```
🔍 Code Highlights
Tool Trait Implementation
rustimpl Tool for WebSearchTool {
    const NAME: &'static str = "web_search";
    type Args = SearchArgs;
    type Output = String;
    type Error = SearchError;

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let results = self.search(&args.query).await?;
        // format and return results...
    }
}
Implementing Rig's Tool trait makes the agent aware of the search capability — this is how LLMs learn to call external functions.

Custom Error Types
rust#[derive(Error, Debug)]
pub enum SearchError {
    #[error("Failed to perform web search: {0}")]
    SearchFailed(String),

    #[error("Rate limited by search provider")]
    RateLimited,

    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),
}
Typed errors give the compiler full visibility into what can fail — making error handling exhaustive and self-documenting.

Config Validation
rustpub fn validate(&self) -> Result<()> {
    if !(0.0..=2.0).contains(&self.temperature) {
        anyhow::bail!("Temperature must be between 0.0 and 2.0");
    }
    if self.max_search_results == 0 {
        anyhow::bail!("MAX_SEARCH_RESULTS must be at least 1");
    }
    Ok(())
}
Validation runs before the agent starts — catching misconfiguration early.

🧪 Tests
bash# Run all tests
cargo test

# Run with output
cargo test -- --nocapture
Tests cover: Config defaults, Config validation, CLI argument parsing, Tool creation, Search result serialization, Domain extraction.

📝 Learning Journey
This project is part of my Rust learning path — focusing on real-world async systems and AI agent architecture.
Rust Concepts Covered

 Ownership & Borrowing
 Async/Await with Tokio
 Trait implementation
 Custom error types with thiserror
 Builder pattern
 CLI tools with clap
 Environment configuration
 Structured logging with tracing
 Unit testing
 Lifetimes in async contexts
 Building REST APIs with Axum
 WebSocket connections


📄 License
This project is licensed under the MIT License.
