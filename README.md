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

Quick Start :

Clone the repo:Bashgit clone [https://github.com/YOUR_USERNAME/ai-research-agent.git](https://github.com/YOUR_USERNAME/ai-research-agent.git)
cd ai-research-agent
Set up Environment:Create a .env file:OLLAMA_HOST=http://localhost:11434
OLLAMA_MODEL=llama3.2
Build and Run:Bashcargo run -- "Explain the benefits of Rust for AI development"
⌨️ CLI UsageFlagLong FlagDescription-q--quickQuick Mode: Fast web search results without AI synthesis.-m--modelOverride the default LLM model.-v--verboseEnable Debug Logs to see the agent's internal reasoning.🛡️ Error HandlingThe project implements a Fail-Fast strategy:thiserror: Used for granular, internal tool errors (e.g., Parsing failures).anyhow: Used for high-level orchestration and user-friendly error reporting.🤝 ContributingContributions are welcome! If you have suggestions for new tools (e.g., GitHub search, PDF parsing), feel free to open an issue or a PR.Developed by AhmadBackend Developer | Rust & Blockchain Enthusiast
