# Melos Compose Skill

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

The bundled `melos` binary is for **macOS x86_64 (Intel)**.

For other platforms, build from the Melos source:

```bash
cargo build --release
cp target/release/melos skill/
```
