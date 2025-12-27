use clap::{Parser, Subcommand};
use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;
use melos::parser::parse;
use melos::walker::walk;
use melos::codegen::generate;
use melos::tui::run_tui;

pub mod inspect;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Compile a Melos file to MIDI
    Compile {
        /// Input Melos file
        #[arg(value_name = "FILE")]
        input: PathBuf,

        /// Output MIDI file
        #[arg(short, long, value_name = "FILE")]
        output: Option<PathBuf>,

        /// Print debug information (AST and IR)
        #[arg(short, long)]
        debug: bool,

        /// Launch interactive TUI mode
        #[arg(short = 'i', long)]
        interactive: bool,
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

    match &cli.command {
        Commands::Compile { input, output, debug, interactive } => {
            // 1. Read Input
            let input_content = fs::read_to_string(input)
                .with_context(|| format!("Failed to read input file: {:?}", input))?;

            // 2. Parse
            let ast = parse(&input_content)
                .context("Failed to parse Melos")?;

            if *debug {
                println!("--- AST ---");
                println!("{:#?}", ast);
            }

            // 3. Walk (AST -> IR)
            let ir = walk(&ast)
                .context("Failed to generate IR")?;

            if *debug {
                println!("--- IR ---");
                println!("{:#?}", ir);
            }

            // Launch TUI if interactive mode requested
            if *interactive {
                return run_tui(input.clone(), ast, ir);
            }

            // 4. Codegen (IR -> MIDI)
            let smf = generate(&ir)
                .context("Failed to generate MIDI")?;

            // 5. Write Output
            let output_path = output.clone().unwrap_or_else(|| {
                let mut p = input.clone();
                p.set_extension("mid");
                p
            });

            smf.save(&output_path)
                .with_context(|| format!("Failed to write MIDI file: {:?}", output_path))?;

            println!("Successfully compiled {:?} to {:?}", input, output_path);
        }
        Commands::Inspect { input } => {
            inspect::inspect(input)?;
        }
    }

    Ok(())
}
