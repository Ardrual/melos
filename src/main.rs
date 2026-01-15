use clap::{Parser, Subcommand};
use anyhow::{Context, Result};
use std::path::PathBuf;
use melos::parser::parse;
use melos::walker::walk;
use melos::codegen::generate;
use melos::loader::load_source;

pub mod inspect;

#[derive(Parser)]
#[command(author, version, about = "Melos - A music composition language", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Input Melos file to compile (shorthand for `melos compile <file>`)
    #[arg(value_name = "FILE")]
    input: Option<PathBuf>,

    /// Output MIDI file
    #[arg(short, long, value_name = "FILE")]
    output: Option<PathBuf>,
}

#[derive(Subcommand)]
enum Commands {
    /// Compile a Melos file or directory to MIDI
    Compile {
        /// Input Melos file or directory containing .mel files
        #[arg(value_name = "PATH")]
        input: PathBuf,

        /// Output MIDI file
        #[arg(short, long, value_name = "FILE")]
        output: Option<PathBuf>,
    },
    /// Inspect a MIDI file
    Inspect {
        /// Input MIDI file
        #[arg(value_name = "FILE")]
        input: PathBuf,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Handle direct file argument (default to compile)
    if let Some(input) = cli.input {
        return compile(&input, cli.output.as_ref());
    }

    // Handle subcommands
    match cli.command {
        Some(Commands::Compile { input, output }) => {
            compile(&input, output.as_ref())
        }
        Some(Commands::Inspect { input }) => {
            inspect::inspect(&input)
        }
        None => {
            // No input and no subcommand - show help
            eprintln!("Usage: melos <FILE> or melos compile <FILE>");
            eprintln!("       melos inspect <FILE.mid>");
            eprintln!();
            eprintln!("Run 'melos --help' for more information.");
            std::process::exit(1);
        }
    }
}

fn compile(input: &PathBuf, output: Option<&PathBuf>) -> Result<()> {
    // 1. Load source (handles both files and directories)
    let loaded = load_source(input)
        .with_context(|| format!("Failed to load source from: {:?}", input))?;

    // 2. Parse
    let ast = parse(&loaded.source)
        .context("Failed to parse Melos")?;

    // 3. Walk (AST -> IR)
    let ir = walk(&ast)
        .context("Failed to generate IR")?;

    // 4. Codegen (IR -> MIDI)
    let smf = generate(&ir)
        .context("Failed to generate MIDI")?;

    // 5. Write Output
    let output_path = output.cloned().unwrap_or_else(|| {
        if loaded.base_path.is_dir() {
            // For directories, create .mid file with directory name
            let dir_name = loaded.base_path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("output");
            loaded.base_path.join(format!("{}.mid", dir_name))
        } else {
            // For files, replace extension
            let mut p = loaded.base_path.clone();
            p.set_extension("mid");
            p
        }
    });

    smf.save(&output_path)
        .with_context(|| format!("Failed to write MIDI file: {:?}", output_path))?;

    println!("Compiled {:?} → {:?}", loaded.base_path, output_path);

    Ok(())
}
