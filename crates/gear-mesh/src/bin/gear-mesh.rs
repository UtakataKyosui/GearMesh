use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::sync::mpsc::channel;
use std::time::Duration;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};

#[derive(Debug, Parser)]
#[command(name = "gear-mesh")]
#[command(about = "GearMesh developer tooling")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Diff two generated TypeScript files and emit a migration guide.
    Diff {
        old: PathBuf,
        new: PathBuf,
        #[arg(long)]
        markdown: bool,
    },
    /// Watch paths and rerun a command when something changes.
    Watch {
        #[arg(long = "path", required = true)]
        paths: Vec<PathBuf>,
        #[arg(long, default_value_t = 250)]
        debounce_ms: u64,
        #[arg(trailing_var_arg = true, required = true)]
        command: Vec<String>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Diff { old, new, markdown } => diff_command(&old, &new, markdown),
        Commands::Watch {
            paths,
            debounce_ms,
            command,
        } => watch_command(&paths, debounce_ms, &command),
    }
}

fn diff_command(old: &PathBuf, new: &PathBuf, markdown: bool) -> Result<()> {
    let old_source =
        fs::read_to_string(old).with_context(|| format!("failed to read {}", old.display()))?;
    let new_source =
        fs::read_to_string(new).with_context(|| format!("failed to read {}", new.display()))?;
    let report = gear_mesh::diff_typescript(&old_source, &new_source);

    if markdown {
        print!(
            "{}",
            report.to_markdown(&old.display().to_string(), &new.display().to_string())
        );
        return Ok(());
    }

    if report.is_empty() {
        println!("No breaking changes detected.");
        return Ok(());
    }

    for change in report.breaking_changes {
        println!("BREAKING: {}", change);
    }
    for change in report.additions {
        println!("ADDED: {}", change);
    }

    Ok(())
}

fn watch_command(paths: &[PathBuf], debounce_ms: u64, command: &[String]) -> Result<()> {
    let (tx, rx) = channel();
    let mut watcher = RecommendedWatcher::new(
        move |event: notify::Result<Event>| {
            let _ = tx.send(event);
        },
        Config::default(),
    )?;

    for path in paths {
        watcher.watch(path, RecursiveMode::Recursive)?;
    }

    if let Err(err) = run_command(command) {
        eprintln!("initial command failed: {err}");
    }

    loop {
        match rx.recv() {
            Ok(Ok(_)) => {
                while rx.recv_timeout(Duration::from_millis(debounce_ms)).is_ok() {}
                if let Err(err) = run_command(command) {
                    eprintln!("watched command failed: {err}");
                }
            }
            Ok(Err(err)) => eprintln!("watch error: {err}"),
            Err(err) => return Err(err.into()),
        }
    }
}

fn run_command(command: &[String]) -> Result<()> {
    let (program, args) = command
        .split_first()
        .context("watch command requires at least one argument")?;

    let status = Command::new(program).args(args).status()?;
    if !status.success() {
        anyhow::bail!("watched command exited with {}", status);
    }

    Ok(())
}
