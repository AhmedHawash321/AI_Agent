// MODULE DECLARATIONS

mod config;
mod agent;
mod tools;

// IMPORTS

use anyhow::Result;
use clap::Parser;
use tracing::{error, info, Level};
use tracing_subscriber::FmtSubscriber;

use crate::agent::ResearchAgent;
use crate::config::Config;

// CLI ARGUMENTS

// Clap's derive feature define CLI arguments as a struct.
// The macros automatically generate argument parsing code.

#[derive(Parser, Debug)]
#[command(
    name = "ai-research-agent",
    author = "Ahmad <ahmedhawash321@gmail.com>",
    version = "0.1.0",
    about = "AI Agent using Ollama & Web Search",
    long_about = "Research any topic locally using LLMs. Requires Ollama and llama3.2."
)]

struct Args {
      /// The research topic or question to investigate
    #[arg(
        help = "The topic to research",
        value_name = "QUERY"
    )]
    query: String,
    
    /// The Ollama model to use (overrides OLLAMA_MODEL env var)
    #[arg(
        short = 'm',
        long = "model",
        help = "Ollama model to use",
        env = "OLLAMA_MODEL"
    )]
    model: Option<String>,
    
    /// Quick search mode - just search, don't synthesize
    #[arg(
        short = 'q',
        long = "quick",
        help = "Quick search mode (no AI synthesis)",
        default_value = "false"
    )]
    quick: bool,
    
    /// Verbose output (debug logging)
    #[arg(
        short = 'v',
        long = "verbose",
        help = "Enable verbose/debug logging",
        default_value = "false"
    )]
    verbose: bool,
}

// MAIN FUNCTION

#[tokio::main]
async fn main() -> Result<()> {
     let args = Args::parse();
      init_logging(args.verbose)?;
      info!("AI Research Agent starting up...");

      let mut config = Config::from_env()?;

      if let Some(model) = args.model {
        info!(model = %model, "Using model from command line");
        config.model = model;
    }

    config.validate()?;

    info!(
        model = %config.model,
        host = %config.ollama_host,
        "Configuration loaded"
    );

    let agent = ResearchAgent::new(config);

    let result = if args.quick {
        // just search, no synthesis
         info!("Running in quick search mode");
        agent.quick_search(&args.query).await
    } else {
        //search + AI synthesis
        info!("Running full research mode");
        agent.research(&args.query).await
    };

     match result {
        Ok(response) => {
            println!("\n{}", "=".repeat(60));
            println!("RESEARCH RESULTS");
            println!("{}\n", "=".repeat(60));
            println!("{}", response);
            println!("\n{}", "=".repeat(60));
        }
         Err(e) => {
             error!(error = %e, "Research failed");
            // Give suggestions based on common errors
             eprintln!("\n❌ Research failed: {}", e);

             if e.to_string().contains("connection refused") {
                eprintln!("\n💡 Tip: Make sure Ollama is running:");
                eprintln!("   ollama serve");
            } else if e.to_string().contains("model") {
                eprintln!("\n💡 Tip: Make sure the model is installed:");
                eprintln!("   ollama pull llama3.2");
            }
            
            // Return the error to set non-zero exit code
            return Err(e);
        }
    }
    info!("Research completed successfully");
    Ok(())
         }
         
fn init_logging(verbose: bool) -> Result<()> {
    // Set log level based on verbose flag
    let level = if verbose { Level::DEBUG } else { Level::INFO };

    let subscriber = FmtSubscriber::builder()
        .with_max_level(level)
        .with_target(true)  // Show the module that logged
        .with_thread_names(false)
        .with_file(false)
        .with_line_number(false)
        .finish();
    
    // Set as the global default
    tracing::subscriber::set_global_default(subscriber)
        .map_err(|e| anyhow::anyhow!("Failed to set logging subscriber: {}", e))?;
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_args_parsing() {
        // Test that CLI args parse correctly
        let args = Args::parse_from(["test", "What is Rust?"]);
        assert_eq!(args.query, "What is Rust?");
        assert!(!args.quick);
        assert!(!args.verbose);
    }
    
    #[test]
    fn test_args_with_flags() {
        let args = Args::parse_from([
            "test",
            "--quick",
            "--verbose",
            "--model", "llama3.2",
            "Test query"
        ]);
        
        assert_eq!(args.query, "Test query");
        assert!(args.quick);
        assert!(args.verbose);
        assert_eq!(args.model, Some("llama3.2".to_string()));
    }
}