# Melos Compose Skill

> **Early Feature**: This is an early-stage feature for teaching AI agents to compose music using Melos. The API and bundled binary may change as the project evolves.

A self-contained skill that lets AI agents compose music in the Melos language.

## Installation

To use this skill with your AI agent, follow the instructions for your specific platform to register custom skills/tools.

Generally, you will need to:
1. Copy the contents of this `skill/` directory to your agent's skill path.
2. Ensure the agent has access to the `melos` binary.

### Example for Claude Code

Copy this folder to your project's `.claude/skills/` directory:

```bash
mkdir -p .claude/skills
cp -r skill .claude/skills/compose
```

## Usage

Once registered, the agent can use the skill to compose music. For example:

- "Compose a jazz piece for piano"
- "Write a string quartet in D minor"
- "Create a short melody"

## Portability

The bundled `melos` binary is built for macOS. For other platforms, build from the Melos source:

```bash
cargo build --release
cp target/release/melos skill/
```
