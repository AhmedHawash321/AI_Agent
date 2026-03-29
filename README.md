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

## 🏗 Architecture

```
ai-research-agent/
├── src/
│   ├── main.rs        # CLI entry point, argument parsing (clap)
│   ├── agent.rs       # ResearchAgent — core LLM orchestration
│   ├── config.rs      # Config struct with env var loading & validation
│   └── tools.rs       # WebSearchTool — DuckDuckGo integration
├── Cargo.toml
└── README.md
```

### How It Works

```
User Query
    ↓
CLI (clap) parses arguments
    ↓
Config loads from .env / environment variables
    ↓
ResearchAgent initialized with Ollama client
    ↓
Agent calls WebSearchTool (DuckDuckGo)
    ↓
LLM synthesizes results into a summary
    ↓
Formatted output printed to terminal
```

---

## 🛠 Features

- 🔍 **Web Search** — DuckDuckGo HTML scraping with multi-strategy parsing
- 🧠 **LLM Synthesis** — Ollama local inference via Rig framework
- ⚡ **Async/Await** — Fully non-blocking with Tokio runtime
- 🛡 **Production Error Handling** — `anyhow` + `thiserror` for typed errors
- ⚙️ **Environment Config** — `.env` support via `dotenvy`
- 🖥 **Rich CLI** — `clap` with flags, env vars, and help text
- 📊 **Structured Logging** — `tracing` + `tracing-subscriber`
- 🧪 **Unit Tests** — Tests for Config, Tools, Agent, and CLI parsing

---

## 🧠 Key Rust Concepts Demonstrated

| Concept | Where Used |
|---|---|
| **Async/Await + Tokio** | `agent.rs`, `tools.rs` — fully async pipeline |
| **Trait Implementation** | `Tool` trait from Rig — enables agent tool use |
| **Custom Error Types** | `thiserror` in `tools.rs` — typed `SearchError` enum |
| **`Box<dyn Error>`** | Dynamic error handling across module boundaries |
| **Struct + impl** | `ResearchAgent`, `Config`, `WebSearchTool` |
| **`#[derive(...)]`** | `Debug`, `Clone`, `Serialize`, `Deserialize`, `Parser` |
| **Builder Pattern** | `ollama_client.agent().preamble().tool().build()` |
| **`Result<T, E>`** | Every fallible function returns `Result` |
| **`?` Operator** | Clean error propagation without boilerplate |
| **Environment Variables** | `std::env::var()` + `dotenvy` for config |
| **Ownership in Async** | `&self` borrows, `.clone()` for tool sharing |
| **Pattern Matching** | `match`, `if let`, `unwrap_or_else` |
| **Iterators** | `.map()`, `.filter()`, `.collect()`, `.enumerate()` |

---

## ⚙️ Tech Stack

| Dependency | Purpose |
|---|---|
| `rig-core 0.27` | LLM agent framework with tool support |
| `tokio` | Async runtime |
| `ollama` (via rig) | Local LLM inference |
| `reqwest` | HTTP client for web search |
| `clap` | CLI argument parsing |
| `serde / serde_json` | Serialization |
| `anyhow` | Flexible error handling |
| `thiserror` | Typed custom errors |
| `tracing` | Structured logging |
| `dotenvy` | `.env` file loading |

---

## 🚀 How to Run

### Prerequisites

- Rust installed → [rustup.rs](https://rustup.rs)
- Ollama installed → [ollama.ai](https://ollama.ai)

### Setup

```bash
# 1. Clone the repo
git clone https://github.com/AhmedHawash321/ai-research-agent.git
cd ai-research-agent

# 2. Pull the LLM model
ollama pull llama3.2

# 3. Start Ollama server
ollama serve

# 4. Create .env file (optional)
cp .env.example .env

# 5. Run the agent
cargo run -- "What is the Rust ownership model?"
```

### Environment Variables

```env
OLLAMA_MODEL=llama3.2
OLLAMA_API_BASE_URL=http://localhost:11434
TEMPERATURE=0.7
MAX_SEARCH_RESULTS=5
RUST_LOG=info
```

### CLI Options

```bash
# Full research (search + AI synthesis)
cargo run -- "Your research topic"

# Quick search only (no AI synthesis)
cargo run -- --quick "Your query"

# Verbose/debug logging
cargo run -- --verbose "Your query"

# Specify model
cargo run -- --model llama3.2 "Your query"
```

---

## 🔍 Code Highlights

### Tool Trait Implementation
```rust
impl Tool for WebSearchTool {
    const NAME: &'static str = "web_search";
    type Args = SearchArgs;
    type Output = String;
    type Error = SearchError;

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let results = self.search(&args.query).await?;
        // format and return results...
    }
}
```
Implementing Rig's `Tool` trait makes the agent aware of the search capability — this is how LLMs learn to call external functions.

---

### Custom Error Types
```rust
#[derive(Error, Debug)]
pub enum SearchError {
    #[error("Failed to perform web search: {0}")]
    SearchFailed(String),

    #[error("Rate limited by search provider")]
    RateLimited,

    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),
}
```
Typed errors give the compiler full visibility into what can fail — making error handling exhaustive and self-documenting.

---

### Config Validation
```rust
pub fn validate(&self) -> Result<()> {
    if !(0.0..=2.0).contains(&self.temperature) {
        anyhow::bail!("Temperature must be between 0.0 and 2.0");
    }
    if self.max_search_results == 0 {
        anyhow::bail!("MAX_SEARCH_RESULTS must be at least 1");
    }
    Ok(())
}
```
Validation runs before the agent starts — catching misconfiguration early.

---

## 🧪 Tests

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture
```

Tests cover: Config defaults, Config validation, CLI argument parsing, Tool creation, Search result serialization, Domain extraction.

---

## 📝 Learning Journey

This project is part of my Rust learning path — focusing on real-world async systems and AI agent architecture.

### Rust Concepts Covered
- [x] Ownership & Borrowing
- [x] Async/Await with Tokio
- [x] Trait implementation
- [x] Custom error types with thiserror
- [x] Builder pattern
- [x] CLI tools with clap
- [x] Environment configuration
- [x] Structured logging with tracing
- [x] Unit testing
- [ ] Lifetimes in async contexts
- [ ] Building REST APIs with Axum
- [ ] WebSocket connections

---

## 📄 License

This project is licensed under the **MIT License**.
