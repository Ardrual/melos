# Melos Compose Skill for Claude Code

A self-contained skill that lets Claude Code compose music in the Melos language.

## Installation

Copy this folder to your project's `.claude/skills/` directory:

```bash
mkdir -p .claude/skills
cp -r /path/to/melos/skill .claude/skills/compose
```

Or if you're in the melos repo:

```bash
mkdir -p .claude/skills
cp -r skill .claude/skills/compose
```

## Usage

In Claude Code, use `/compose` or ask naturally:

- "compose a jazz piece for piano"
- "write a string quartet in D minor"
- "create a short melody"

## Platform

The bundled `melos` binary is for **macOS x86_64 (Intel)**.

For other platforms, build from the melos repo:

```bash
cargo build --release
cp target/release/melos /path/to/skill/
```
