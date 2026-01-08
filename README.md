# Melos


**Melos** is a high-level, human-readable notation language for describing Western music. It compiles directly to MIDI and is designed to be intuitive for both humans and LLMs, and to enable more effective LLM composition.

## Features

- **Structural Hierarchy**: Captures music as Sections, Parts, and Bars, not just a stream of MIDI events.
- **Readable Syntax**: Designed to look like a clean, stripped-down score.
- **Direct Compilation**: Compiles `.mel` source files to standard MIDI (`.mid`) files.

## Installation

Melos is built with Rust. You'll need `cargo` to build and run it.

```bash
cargo build --release
```

## Usage

### Compiling a Score

To compile a `.mel` file to MIDI:

```bash
cargo run --release -- compile programs/myscore.mel
```

This will generate `myscore.mid` in the same directory.

### Syntax Example

file: `suite.mel`

```mel
Title: "Suite in G"
Tempo: 120
Time: 4/4
Key: G "Major"

Part: "Piano" "Acoustic Grand Piano"
    | G4 q  B4 q  D5 h |
    | C5 q  E5 q  G5 h |
    | D5 q  F#5 q A5 q G5 q |
    | G5 w |
```

### Using with LLMs

Melos is designed to be easy for LLMs to generate. As a simple approach, you can upload the LLM user guide at the start of a chat, and most models will be able to generate valid Melos code, and discuss it at a high level with general music theory knowledge.

## Project Structure

- `compiler/`: The Rust compiler (Melos -> MIDI)
- `programs/`: Example scores and test files

## Development

Run the test suite:

```bash
cargo test
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE.md) file for details.
