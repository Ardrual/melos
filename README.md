# Melos

**Melos** is a high-level, human-readable notation language for describing Western music. It compiles directly to MIDI and is designed to be intuitive for both humans and LLMs, enabling more effective AI-assisted composition.

## Features

- **Structural Hierarchy**: Captures music as Sections, Parts, and Bars, rather than just a stream of MIDI events.
- **Readable Syntax**: Designed to look like a clean, stripped-down score.
- **Direct Compilation**: Compiles `.mel` source files to standard MIDI (`.mid`) files.
- **LLM Optimized**: Syntax and structure are tailored for generation by Large Language Models.

## Installation

Melos is built with Rust. You'll need `cargo` to build and run it.

```bash
cargo build --release
```

## Usage

### Compiling a Score

To compile a `.mel` file to MIDI:

```bash
cargo run --release -- compile scores/myscore.mel
```

This will generate `myscore.mid` in the same directory.

### Syntax Example

file: `suite.mel`

```mel
Title: "Suite in G"
Tempo: 120
Time: 4/4
Key: G "Major"

Part: "Piano" Instrument: Piano {
    | G4 q  B4 q  D5 h |
    | C5 q  E5 q  G5 h |
    | D5 q  F#5 q A5 q G5 q |
    | G5 w |
}
```

### AI Integration

Melos is specifically designed for AI workflows.

- **Composer Skill**: We provide a portable "skill" definition in the `skill/` directory that can be used to teach AI agents (like Claude Code) how to compose, compile, and debug music using Melos.
- **User Guide**: Check out the [LLM User Guide](LLM_USER_GUIDE.md) for a comprehensive look at the language features and tips for LLM prompting.

## Project Structure

- `src/`: The Rust compiler source code.
- `skill/`: A portable AI agent skill for music composition. *(Early feature - see `skill/README.md`)*
- `scores/`: Example scores and test files.
- `tests/`: Integration tests for the compiler.

## Development

Run the test suite:

```bash
cargo test
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE.md) file for details.
