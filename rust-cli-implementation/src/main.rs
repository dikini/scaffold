use clap::{Parser, Subcommand};
use std::path::PathBuf;
use rust_cli_example::{process_text, write_output, Transformation};

#[derive(Parser)]
#[command(name = "rust-cli-example")]
#[command(about = "A command-line tool for processing text files with various transformations")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Convert text to uppercase
    Upper {
        /// Input file (stdin if not specified)
        input: Option<PathBuf>,

        /// Output file (stdout if not specified)
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    /// Convert text to lowercase
    Lower {
        /// Input file (stdin if not specified)
        input: Option<PathBuf>,

        /// Output file (stdout if not specified)
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    /// Remove duplicate lines
    Dedup {
        /// Input file (stdin if not specified)
        input: Option<PathBuf>,

        /// Output file (stdout if not specified)
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Upper { input, output } => {
            let input_path = input.as_ref().map(|p| p.to_string_lossy().to_string());
            let result = process_text(input_path.as_deref(), Transformation::Upper)?;
            write_output(output.as_ref().map(|p| p.to_string_lossy().to_string()).as_deref(), &result)?;
        }
        Commands::Lower { input, output } => {
            let input_path = input.as_ref().map(|p| p.to_string_lossy().to_string());
            let result = process_text(input_path.as_deref(), Transformation::Lower)?;
            write_output(output.as_ref().map(|p| p.to_string_lossy().to_string()).as_deref(), &result)?;
        }
        Commands::Dedup { input, output } => {
            let input_path = input.as_ref().map(|p| p.to_string_lossy().to_string());
            let result = process_text(input_path.as_deref(), Transformation::Dedup)?;
            write_output(output.as_ref().map(|p| p.to_string_lossy().to_string()).as_deref(), &result)?;
        }
    }

    Ok(())
}
