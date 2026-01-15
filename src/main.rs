use clap::{Parser, Subcommand};
use anyhow::{Context, Result};
use std::path::PathBuf;
use melos::parser::parse;
use melos::walker::walk;
use melos::codegen::generate;
use melos::tui::run_tui;
use melos::gui::run_gui;
use melos::loader::load_source;

pub mod inspect;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
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

        /// Print debug information (AST and IR)
        #[arg(short, long)]
        debug: bool,

        /// Launch interactive TUI mode
        #[arg(short = 'i', long)]
        interactive: bool,

        /// Launch GUI piano roll view
        #[arg(short = 'g', long)]
        gui: bool,
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
        Commands::Compile { input, output, debug, interactive, gui } => {
            // 1. Load source (handles both files and directories)
            let loaded = load_source(input)
                .with_context(|| format!("Failed to load source from: {:?}", input))?;

            // 2. Parse
            let ast = parse(&loaded.source)
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
                return run_tui(loaded.base_path.clone(), ast, ir);
            }

            // Launch GUI if gui mode requested
            if *gui {
                return run_gui(loaded.base_path.clone(), ast, ir);
            }

            // 4. Codegen (IR -> MIDI)
            let smf = generate(&ir)
                .context("Failed to generate MIDI")?;

            // 5. Write Output
            let output_path = output.clone().unwrap_or_else(|| {
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

            println!("Successfully compiled {:?} to {:?}", loaded.base_path, output_path);
        }
        Commands::Inspect { input } => {
            inspect::inspect(input)?;
        }
    }

    Ok(())
}
