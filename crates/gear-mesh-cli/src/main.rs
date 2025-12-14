//! gear-mesh CLI
//!
//! TypeScript型定義生成のためのコマンドラインツール

use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, Subcommand};

mod config;
mod generate;
mod watch;

use config::Config;

#[derive(Parser)]
#[command(name = "gear-mesh")]
#[command(author, version, about = "Next-generation Rust to TypeScript type definition sharing", long_about = None)]
struct Cli {
    /// Configuration file path
    #[arg(short, long, default_value = "gear-mesh.toml")]
    config: PathBuf,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate TypeScript type definitions
    Generate {
        /// Input directory containing Rust source files
        #[arg(short, long)]
        input: Option<PathBuf>,

        /// Output directory for TypeScript files
        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    /// Watch for file changes and regenerate
    Watch {
        /// Input directory to watch
        #[arg(short, long)]
        input: Option<PathBuf>,

        /// Output directory for TypeScript files
        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    /// Initialize a new gear-mesh.toml configuration file
    Init,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // 設定ファイルを読み込む（存在する場合）
    let config = if cli.config.exists() {
        Config::load(&cli.config)?
    } else {
        Config::default()
    };

    match cli.command {
        Commands::Generate { input, output } => {
            let input_dir = input.unwrap_or_else(|| config.input.clone());
            let output_dir = output.unwrap_or_else(|| config.output.clone());
            generate::run(&input_dir, &output_dir, &config)?;
        }
        Commands::Watch { input, output } => {
            let input_dir = input.unwrap_or_else(|| config.input.clone());
            let output_dir = output.unwrap_or_else(|| config.output.clone());
            watch::run(&input_dir, &output_dir, &config)?;
        }
        Commands::Init => {
            config::init_config()?;
        }
    }

    Ok(())
}
