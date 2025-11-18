mod parser;
mod generator;

use anyhow::{Context, Result};
use clap::Parser;
use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;

use crate::generator::TypeScriptGenerator;
use crate::parser::ApexParser;

#[derive(Parser, Debug)]
#[command(name = "apex-ts")]
#[command(about = "Generate TypeScript type definitions from Apex classes with @tsexport annotation", long_about = None)]
struct Cli {
    /// Input directory containing Apex class files (.cls)
    #[arg(short, long, value_name = "DIR")]
    input: PathBuf,

    /// Output TypeScript file path
    #[arg(short, long, value_name = "FILE", default_value = "types.d.ts")]
    output: PathBuf,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // tracing の初期化
    if cli.verbose {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .init();
    } else {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::WARN)
            .init();
    }

    if cli.verbose {
        println!("Scanning directory: {}", cli.input.display());
    }

    // .cls ファイルを収集
    let apex_files: Vec<String> = WalkDir::new(&cli.input)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "cls"))
        .map(|e| e.path().to_string_lossy().to_string())
        .collect();

    if cli.verbose {
        println!("Found {} Apex class files", apex_files.len());
    }

    if apex_files.is_empty() {
        println!("No Apex class files (.cls) found in {}", cli.input.display());
        return Ok(());
    }

    // Apex クラスを解析
    let parser = ApexParser::new()?;
    let classes = parser.parse_files(&apex_files)?;

    if cli.verbose {
        println!("Found {} classes with @tsexport annotation", classes.len());
    }

    if classes.is_empty() {
        println!("No classes with @tsexport annotation found");
        return Ok(());
    }

    // TypeScript 型定義を生成
    let generator = TypeScriptGenerator::new();
    let typescript_code = generator.generate(&classes);

    // ファイルに書き込み
    if let Some(parent) = cli.output.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create output directory: {}", parent.display()))?;
    }

    fs::write(&cli.output, typescript_code)
        .with_context(|| format!("Failed to write output file: {}", cli.output.display()))?;

    println!(
        "✓ Successfully generated TypeScript definitions: {}",
        cli.output.display()
    );
    println!("  {} interface(s) generated", classes.len());

    Ok(())
}
