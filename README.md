# 🤖 Autonomous AI Research Agent (Rust + Rig)

A high-performance, local-first AI Research Agent built with **Rust**. This agent leverages the **Rig** framework and **Ollama** to perform deep web research, synthesize information, and provide structured reports—all from your terminal.

![Rust](https://img.shields.io/badge/language-Rust-orange.svg)
![LLM](https://img.shields.io/badge/LLM-Llama_3.2-blue.svg)
![Framework](https://img.shields.io/badge/Framework-Rig-green.svg)

---

## 🚀 Overview

This project goes beyond simple LLM wrappers. It implements a true **Agentic Workflow** capable of:
- **Autonomous Web Searching:** Using a custom-built scraper for DuckDuckGo.
- **Multi-Turn Reasoning:** The agent can iterate up to 5 times to refine its research.
- **Smart Routing:** Optimized handling for math and basic greetings to save compute.
- **Local-First Privacy:** Runs entirely on your machine using Ollama.

## 🛠️ Tech Stack

- **Core:** [Rust](https://www.rust-lang.org/) (Safety & Performance)
- **Agent Framework:** [Rig](https://github.com/0xPlayground/rig)
- **Inference Engine:** [Ollama](https://ollama.ai/) (Llama 3.2)
- **Async Runtime:** [Tokio](https://tokio.rs/)
- **CLI Power:** [Clap](https://docs.rs/clap/latest/clap/)
- **Logging & Trace:** [Tracing](https://docs.rs/tracing/latest/tracing/)

---

## 🏗️ Architecture

The project is designed with a modular architecture for scalability:
- `tools/`: Specialized scrapers and search utilities with built-in rate limiting.
- `agent/`: High-level logic for the Research Agent and Tool integration.
- `config/`: Centralized configuration management with `.env` and CLI overrides.
- `main.rs`: Robust entry point with structured error handling and logging.



---

🚀 Quick Start
Prerequisites
Install Rust (if not already installed):

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
Install Ollama:

Visit ollama.ai and follow installation instructions
Or on Linux: curl -fsSL https://ollama.com/install.sh | sh
Pull a model:

ollama pull llama3.2
# Or any other model you prefer:
# ollama pull deepseek-v3.2
# ollama pull qwen3-coder
Start Ollama:

ollama serve
Installation
# Clone the repository
git clone https://github.com/aarambh-darshan/ai-research-agent.git
cd ai-research-agent

# Copy environment template
cp .env.example .env

# Build the project
cargo build --release
Usage
# Basic research query
cargo run -- "What are the latest developments in Rust async runtime?"

# Quick search mode (no AI synthesis)
cargo run --release -- --quick "Rust web frameworks 2024"

# Use a specific model
cargo run -- --model deepseek-v3.2 "Machine learning in Rust"

# Verbose output
cargo run -- --verbose "WebAssembly trends"

# Show help
cargo run -- --help
📁 Project Structure
ai-research-agent/
├── Cargo.toml          # Project dependencies and metadata
├── .env.example        # Environment variable template
├── README.md           # This file
└── src/
    ├── main.rs         # CLI entry point and application logic
    ├── config.rs       # Configuration management
    ├── agent.rs        # Research agent implementation
    └── tools.rs        # Web search tool (DuckDuckGo)
🔧 Configuration
Edit .env to customize the agent:

# Model to use (must be installed in Ollama)
OLLAMA_MODEL=llama3.2

# Ollama server URL
OLLAMA_HOST=http://localhost:11434

# Response creativity (0.0 = focused, 1.0 = creative)
TEMPERATURE=0.7

# Number of search results to analyze
MAX_SEARCH_RESULTS=5

# Logging level
RUST_LOG=info
🎓 Learning Rust Concepts
This codebase demonstrates these Rust concepts with inline comments:

Concept	File	Description
Structs & Enums	config.rs	Data types and pattern matching
Traits	tools.rs	Implementing the Rig Tool trait
Ownership & Borrowing	agent.rs	Memory safety without GC
Async/Await	agent.rs, tools.rs	Non-blocking I/O
Error Handling	All files	Result, ? operator, anyhow
Derive Macros	All files	Debug, Clone, Serialize
Unit Tests	All files	The #[cfg(test)] pattern
🛠️ Extending the Agent
Adding a New Tool
Create a new struct in tools.rs:

pub struct MyNewTool {
    // fields
}
Implement the Tool trait:

impl Tool for MyNewTool {
    const NAME: &'static str = "my_tool";
    // ... implement required methods
}
Register with the agent in agent.rs:

let agent = client
    .agent(&model)
    .tool(web_search_tool)
    .tool(my_new_tool)  // Add here
    .build();
Using Different Models
Any Ollama-compatible model works:

ollama pull mistral
ollama pull codellama
ollama pull gemma2
Then set OLLAMA_MODEL in .env or use --model flag.

🧪 Testing
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_config
📊 Sample Output
$ cargo run -- "What is WebAssembly?"

============================================================
RESEARCH RESULTS
============================================================

## Overview
WebAssembly (Wasm) is a binary instruction format designed for...

## Key Findings
1. **Performance**: Near-native execution speed...
2. **Portability**: Runs on any platform with a Wasm runtime...
3. **Security**: Sandboxed execution environment...

## Sources
- https://webassembly.org/
- https://developer.mozilla.org/en-US/docs/WebAssembly
- ...

============================================================
🐛 Troubleshooting
"Connection refused" error
Make sure Ollama is running:

ollama serve
"Model not found" error
Pull the model first:

ollama pull llama3.2
Slow responses
Try a smaller model: ollama pull gemma2:2b
Check your hardware - LLMs need significant RAM/VRAM

🙏 Acknowledgments
Rig Framework - The Rust AI framework
Ollama - Local LLM runner
DuckDuckGo - Privacy-respecting search
