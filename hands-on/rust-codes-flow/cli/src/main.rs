//! RustCodeFlow CLI
//!
//! CLI tool for converting Rust repositories to JSONL datasets for AI training.

use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use walkdir::WalkDir;

use core::*;

#[derive(Parser, Debug)]
#[command(name = "rustcodeflow")]
#[command(about = "Convert Rust repositories to JSONL datasets for AI training")]
struct Cli {
    /// Source: GitHub URL or local path to Rust repository
    source: String,

    /// Output JSONL file path
    #[arg(short, long)]
    output: PathBuf,

    /// Preserve documentation comments in output
    #[arg(long, default_value = "false")]
    keep_docs: bool,

    /// Include 50 lines of context before/after each item
    #[arg(long, default_value = "false")]
    full_context: bool,

    /// Number of parallel threads (default: logical CPUs)
    #[arg(long, default_value_t = num_cpus::get())]
    threads: usize,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    println!("🚀 RustCodeFlow: Converting Rust repositories to AI datasets");
    println!("📁 Source: {}", cli.source);
    println!("📄 Output: {}", cli.output.display());
    println!(
        "🔧 Options: docs={}, context={}, threads={}",
        cli.keep_docs, cli.full_context, cli.threads
    );

    // Set up thread pool
    rayon::ThreadPoolBuilder::new()
        .num_threads(cli.threads)
        .build_global()?;

    // Create configuration
    let config = ExtractConfig {
        keep_docs: cli.keep_docs,
        include_context: cli.full_context,
        context_lines: 50,
    };

    // Clone repository or verify local path
    let repo_path = clone_or_verify_repo(&cli.source)?;

    // Get repository metadata
    let (repo_url, commit_hash) = get_repo_metadata(&repo_path)?;

    // Find all Rust files
    let rust_files = find_rust_files(&repo_path)?;

    if rust_files.is_empty() {
        anyhow::bail!("No .rs files found in repository");
    }

    println!("📚 Found {} Rust files to process", rust_files.len());

    // Set up progress bar
    let progress_style = ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} files ({eta})")?
        .progress_chars("█░");

    let progress_bar = ProgressBar::new(rust_files.len() as u64);
    progress_bar.set_style(progress_style.clone());

    // Process files in parallel
    let mut all_items = Vec::new();

    for (index, file_path) in rust_files.iter().enumerate() {
        progress_bar.set_position(index as u64);
        match extract_items_from_file(
            file_path,
            repo_url.to_string(),
            commit_hash.to_string(),
            &config,
        ) {
            Ok(items) => all_items.extend(items),
            Err(e) => {
                eprintln!("Warning: Failed to process {}: {}", file_path.display(), e);
            }
        }
    }

    progress_bar.finish_with_message("✅ Processing complete!");

    println!("📊 Extracted {} code items", all_items.len());

    // Normalize items (identifier anonymization, docstring extraction)
    println!("🔧 Normalizing identifiers and extracting metadata...");
    normalize_items(&mut all_items, &config)?;

    // Generate semantic hashes
    println!("🔐 Generating semantic hashes for deduplication...");
    hash_and_update_items(&mut all_items)?;

    // Write to JSONL
    println!("💾 Writing to JSONL file...");
    write_jsonl(&all_items, &cli.output)?;

    // Print summary statistics
    print_summary(&all_items)?;

    println!("✨ Dataset generation complete!");
    println!("📁 Output: {}", cli.output.display());

    Ok(())
}

/// Clone GitHub repository or verify local path exists
fn clone_or_verify_repo(source: &str) -> anyhow::Result<PathBuf> {
    let repo_name = extract_repo_name(source)?;
    let temp_dir = std::env::temp_dir().join(format!("rustcodeflow_{}", repo_name));

    if source.starts_with("http") {
        // GitHub URL - clone or update
        if temp_dir.exists() {
            println!("📂 Updating existing repository at {}", temp_dir.display());
            let output = std::process::Command::new("git")
                .args(["pull", "origin", "main"])
                .current_dir(&temp_dir)
                .output()?;

            if !output.status.success() {
                println!("⚠️  Git pull failed, trying master branch...");
                let output = std::process::Command::new("git")
                    .args(["pull", "origin", "master"])
                    .current_dir(&temp_dir)
                    .output()?;

                if !output.status.success() {
                    anyhow::bail!(
                        "Failed to update repository: {}",
                        String::from_utf8_lossy(&output.stderr)
                    );
                }
            }
        } else {
            println!("📥 Cloning repository...");
            let output = std::process::Command::new("git")
                .args(["clone", "--depth", "1", source, temp_dir.to_str().unwrap()])
                .output()?;

            if !output.status.success() {
                anyhow::bail!(
                    "Failed to clone repository: {}",
                    String::from_utf8_lossy(&output.stderr)
                );
            }
        }

        Ok(temp_dir)
    } else {
        // Local path
        let path = PathBuf::from(source);
        if !path.exists() {
            anyhow::bail!("Local path does not exist: {}", source);
        }
        Ok(path)
    }
}

/// Extract repository name from URL or path
fn extract_repo_name(source: &str) -> anyhow::Result<String> {
    if let Some(path) = PathBuf::from(source).file_name() {
        Ok(path.to_string_lossy().to_string())
    } else {
        // Extract from GitHub URL
        if let Some(last_slash) = source.rfind('/') {
            let repo_part = &source[last_slash + 1..];
            if let Some(dot_git) = repo_part.find(".git") {
                Ok(repo_part[..dot_git].to_string())
            } else {
                Ok(repo_part.to_string())
            }
        } else {
            Ok("unknown_repo".to_string())
        }
    }
}

/// Get repository metadata (URL and commit hash)
fn get_repo_metadata(repo_path: &PathBuf) -> anyhow::Result<(String, String)> {
    // Get commit hash
    let output = std::process::Command::new("git")
        .args(["rev-parse", "HEAD"])
        .current_dir(repo_path)
        .output()?;

    if !output.status.success() {
        anyhow::bail!(
            "Failed to get commit hash: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    let commit_hash = String::from_utf8_lossy(&output.stdout).trim().to_string();

    // Try to get remote URL
    let output = std::process::Command::new("git")
        .args(["remote", "get-url", "origin"])
        .current_dir(repo_path)
        .output()?;

    let repo_url = if output.status.success() {
        String::from_utf8_lossy(&output.stdout).trim().to_string()
    } else {
        "local://".to_string() + &repo_path.display().to_string()
    };

    Ok((repo_url, commit_hash))
}

/// Find all .rs files in repository
fn find_rust_files(repo_path: &PathBuf) -> anyhow::Result<Vec<PathBuf>> {
    let mut rust_files = Vec::new();

    for entry in WalkDir::new(repo_path) {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("rs") {
            rust_files.push(path.to_path_buf());
        }
    }

    rust_files.sort();
    Ok(rust_files)
}

/// Write items to JSONL file
fn write_jsonl(items: &[ExtractedItem], output_file: &PathBuf) -> CoreResult<()> {
    let file = fs::File::create(output_file)?;
    let mut writer = std::io::BufWriter::new(file);

    for item in items {
        let json_line = serde_json::to_string(item)?;
        writeln!(writer, "{}", json_line)?;
    }

    Ok(())
}

/// Print summary statistics
fn print_summary(items: &[ExtractedItem]) -> CoreResult<()> {
    use std::collections::HashMap;

    let mut kind_counts = HashMap::new();
    let mut total_lines = 0;

    for item in items {
        *kind_counts.entry(&item.item_meta.kind).or_insert(0) += 1;
        total_lines += (item.item_meta.end_line - item.item_meta.start_line + 1) as usize;
    }

    println!("\n📈 Summary Statistics:");
    println!("  Total items: {}", items.len());
    println!("  Total lines of code: {}", total_lines);
    println!(
        "  Average lines per item: {:.1}",
        total_lines as f64 / items.len() as f64
    );

    println!("\n📋 Item breakdown:");
    for (kind, count) in kind_counts {
        println!("  {}: {}", kind, count);
    }

    // Hash statistics
    let hash_stats = HashStats::analyze(items)?;
    hash_stats.print();

    Ok(())
}
