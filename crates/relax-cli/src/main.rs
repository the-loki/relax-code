use std::error::Error;
use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};
use relax_core::build_system_prompt;
use relax_runtime::{SessionStore, SkillLoader};

#[derive(Parser)]
#[command(name = "relax")]
#[command(about = "A local AI coding agent CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Chat(ChatArgs),
    Resume(ResumeArgs),
}

#[derive(Args)]
struct ChatArgs {
    #[arg(long)]
    workspace: Option<PathBuf>,
    #[arg(long = "skill")]
    skills: Vec<String>,
}

#[derive(Args)]
struct ResumeArgs {
    #[arg(long)]
    session: String,
    #[arg(long)]
    workspace: Option<PathBuf>,
}

fn main() {
    if let Err(error) = run(Cli::parse()) {
        eprintln!("{error}");
        std::process::exit(1);
    }
}

fn run(cli: Cli) -> Result<(), Box<dyn Error + Send + Sync>> {
    match cli.command {
        Commands::Chat(args) => {
            let workspace = match args.workspace {
                Some(path) => path,
                None => std::env::current_dir()?,
            };
            let loader = SkillLoader::from_workspace(&workspace);
            let mut skill_texts = Vec::new();

            for skill_name in args.skills {
                skill_texts.push(loader.load(&skill_name)?);
            }

            let _system_prompt = build_system_prompt(&skill_texts);
            Ok(())
        }
        Commands::Resume(args) => {
            let workspace = match args.workspace {
                Some(path) => path,
                None => std::env::current_dir()?,
            };
            let store = SessionStore::new(workspace);
            let session = store.load(&args.session)?;
            println!(
                "Loaded session {} ({} messages)",
                args.session,
                session.messages().len()
            );
            Ok(())
        }
    }
}
