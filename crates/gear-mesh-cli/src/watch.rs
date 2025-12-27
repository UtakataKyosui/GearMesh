//! ウォッチモードコマンド

use std::path::Path;
use std::sync::mpsc::channel;
use std::time::Duration;

use anyhow::{Context, Result};
use notify::{Config as NotifyConfig, RecommendedWatcher, RecursiveMode, Watcher};

use crate::config::Config;
use crate::generate;

/// ウォッチモードを実行
pub fn run(input_dir: &Path, output_dir: &Path, config: &Config) -> Result<()> {
    println!("Starting watch mode...");
    println!("  Watching: {}", input_dir.display());
    println!("  Output: {}", output_dir.display());
    println!("Press Ctrl+C to stop.\n");

    // 初回生成
    generate::run(input_dir, output_dir, config)?;

    // ファイル監視を設定
    let (tx, rx) = channel();

    let mut watcher = RecommendedWatcher::new(
        move |res| {
            if let Ok(event) = res {
                let _ = tx.send(event);
            }
        },
        NotifyConfig::default().with_poll_interval(Duration::from_millis(500)),
    )
    .context("Failed to create file watcher")?;

    watcher
        .watch(input_dir, RecursiveMode::Recursive)
        .context("Failed to watch input directory")?;

    // イベントループ
    loop {
        match rx.recv() {
            Ok(event) => {
                // Rustファイルの変更のみを処理
                let is_rust_file = event
                    .paths
                    .iter()
                    .any(|p| p.extension().map(|ext| ext == "rs").unwrap_or(false));

                if is_rust_file {
                    println!("\nFile changed, regenerating...");
                    if let Err(e) = generate::run(input_dir, output_dir, config) {
                        eprintln!("Error: {}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("Watch error: {}", e);
                break;
            }
        }
    }

    Ok(())
}
