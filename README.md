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

| Component | Technology | Role |
| :--- | :--- | :--- |
| **Language** | **Rust** | Safety, Memory Management, & Speed |
| **Agent Engine** | **Rig Framework** | Tool invocation & LLM orchestration |
| **Local LLM** | **Ollama (Llama 3.2)** | Local inference & reasoning |
| **Async Runtime** | **Tokio** | Non-blocking I/O for web & AI |
| **CLI Parser** | **Clap** | Professional Command Line Interface |

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

🎓 Learning Rust Concepts
This project is a practical guide to advanced Rust:

✅ Async/Await: Handling concurrent web requests.

✅ Error Handling: Using anyhow for apps and thiserror for tools.

✅ Traits: Implementing Rig's Tool trait for extensibility.

✅ Memory Safety: Ownership & Borrowing in complex agent states.

🔧 Configuration (.env)
Customize your agent's behavior:

OLLAMA_MODEL: Default LLM (e.g., llama3.2).

TEMPERATURE: Creativity level (0.0 - 1.0).

MAX_SEARCH_RESULTS: How many sources to analyze.

🤝 Contributing & Troubleshooting
Getting "Connection Refused"? Ensure ollama serve is running in the background.

Want to add tools? Check src/tools.rs and implement the Tool trait for GitHub, PDFs, or Databases!

Developed by Ahmad Backend Developer | Rust & Blockchain Enthusiast
