mod commands;
mod db;
mod formatter;

use anyhow::Result;
use clap::{CommandFactory, Parser, Subcommand, ValueEnum};
use clap_complete::Shell;
use std::fmt;

/// Supported application types.  Using an enum means Clap validates the value,
/// produces a smart error for invalid input, and generates correct shell
/// completions automatically.
#[derive(Debug, Clone, Copy, ValueEnum)]
#[value(rename_all = "lowercase")]
pub enum AppType {
    Claude,
    Opencode,
    Openclaw,
    Codex,
    Gemini,
}

impl fmt::Display for AppType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppType::Claude => write!(f, "claude"),
            AppType::Opencode => write!(f, "opencode"),
            AppType::Openclaw => write!(f, "openclaw"),
            AppType::Codex => write!(f, "codex"),
            AppType::Gemini => write!(f, "gemini"),
        }
    }
}

#[derive(Parser, Debug)]
#[command(
    name = "ccswitch-cli",
    about = "CLI for managing CC Switch providers",
    version
)]
struct Cli {
    #[arg(long, help = "AI-friendly output mode (XML)")]
    ai: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    #[command(about = "List providers for an app")]
    List {
        #[arg(help = "App type: claude, opencode, openclaw, codex, gemini")]
        app: AppType,
    },
    #[command(about = "Switch to a different provider")]
    Switch {
        #[arg(help = "App type: claude, opencode, openclaw, codex, gemini")]
        app: AppType,
        #[arg(long, short, help = "Provider ID to switch to")]
        provider: String,
        #[arg(long, help = "Dry run - show what would change")]
        dry_run: bool,
        #[arg(long, help = "Confirmation token for AI mode")]
        confirm: Option<String>,
    },
    #[command(about = "Show current provider for an app")]
    Current {
        #[arg(help = "App type: claude, opencode, openclaw, codex, gemini")]
        app: AppType,
    },
    #[command(about = "Check provider health")]
    Health {
        #[arg(help = "App type: claude, opencode, openclaw, codex, gemini")]
        app: AppType,
    },
    #[command(about = "Generate shell completions")]
    Completions {
        #[arg(help = "Shell to generate completions for (bash, elvish, fish, powershell, zsh)")]
        shell: Shell,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let result = match &cli.command {
        Commands::List { app } => commands::list::execute(&app.to_string(), cli.ai),
        Commands::Switch {
            app,
            provider,
            dry_run,
            confirm,
        } => commands::switch::execute(
            &app.to_string(),
            provider,
            *dry_run,
            confirm.as_deref(),
            cli.ai,
        ),
        Commands::Current { app } => commands::current::execute(&app.to_string(), cli.ai),
        Commands::Health { app } => commands::health::execute(&app.to_string(), cli.ai),
        Commands::Completions { shell } => {
            let mut cmd = Cli::command();
            clap_complete::generate(*shell, &mut cmd, "ccswitch-cli", &mut std::io::stdout());
            Ok(())
        }
    };

    if let Err(e) = result {
        if cli.ai {
            println!(
                "<ccswitch error=\"{}\"><message>{}</message></ccswitch>",
                formatter::ai::escape_xml(&e.root_cause().to_string()),
                formatter::ai::escape_xml(&e.root_cause().to_string())
            );
        } else {
            eprintln!("Error: {}", e.root_cause());
        }
        std::process::exit(1);
    }

    Ok(())
}
