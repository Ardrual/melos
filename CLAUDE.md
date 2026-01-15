# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Melos is a domain-specific language for describing Western music that compiles to MIDI. It's designed to be human-readable and LLM-friendly.

## Build and Test Commands

```bash
# Build
cargo build --release

# Run tests
cargo test

# Run a single test
cargo test <test_name>

# Compile a .mel file to MIDI
cargo run --release -- compile scores/myscore.mel

# Compile all .mel files in a directory (concatenates them into one MIDI)
cargo run --release -- compile scores/myproject/

# Inspect a MIDI file
cargo run --release -- inspect file.mid
```

## Architecture

The compiler follows a traditional pipeline: **Source → Parse → AST → Walk → IR → Codegen → MIDI**

### Key Modules

- **`grammar.pest`**: PEG grammar defining Melos syntax (uses pest parser generator)
- **`parser.rs`**: Transforms pest parse tree into AST
- **`ast.rs`**: AST types - `Score`, `Part`, `Measure`, `Event`, `Note`, `Pitch`, `Duration`, etc.
- **`walker.rs`**: Transforms AST to IR, handles timing calculation (PPQ=480), velocity mapping, measure validation
- **`ir.rs`**: Intermediate representation with absolute tick times - `IrScore`, `IrTrack`, `IrEvent`
- **`codegen.rs`**: Generates MIDI using the `midly` crate, converts absolute to delta times
- **`instruments.rs`**: Maps instrument names to General MIDI program numbers

### Data Flow

1. `parse()` → `Score` (AST with headers and parts containing measures)
2. `walk()` → `IrScore` (tracks with absolute-timed events, conductor track for tempo/time sig)
3. `generate()` → `Smf` (MIDI file with delta times)

## Development Practices

- Follow TDD principles when feasible: write tests first, then implement the code to make them pass.
- Commit frequently to maintain clear, incremental history.

## Melos Language Reference

See `LLM_USER_GUIDE.md` for complete syntax. Key points:

- Headers: `Title:`, `Tempo:`, `Time:`, `Key:`
- Parts: `Part: "Name" Instrument: InstrumentName { measures }`
- Measures enclosed in pipes: `| C4 q D4 q E4 h |`
- Durations: `w`(whole), `h`(half), `q`(quarter), `e`(eighth), `s`(sixteenth), with dots for extensions
- Dynamics: `ppp` through `fff`
- Chords: `[C4 E4 G4] q`
- Tuplets: `Tuplet(3:2) { E4 q E4 q E4 q }`
- Swing: `Swing: e 0.66` (applies swing to eighth notes)
- Comments: `// comment` or `= comment` (line comments)

> **Note**: The `skill/` directory contains an early-stage portable AI skill for music composition. See `skill/README.md` for details.
