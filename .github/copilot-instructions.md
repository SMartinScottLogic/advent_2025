# Copilot Coding Agent Instructions for `advent_2025`

## Project Overview
- **Monorepo for Advent of Code 2025**: Each day is a separate Rust crate (e.g., `day1`, `day2`, ...), with shared utilities in `utils/` and project-level orchestration in `xtask/`.
- **Input files**: All puzzle inputs are in `input/` (e.g., `day1.full`, `day1.sample`).
- **Template**: New days are bootstrapped from `template/`.

## Key Workflows
- **Setup a new day**: Use the custom xtask:
  ```sh
  cargo xtask setup day${N}
  ```
  This creates a new day crate from the template. Does NOT commit to git.
- **Build & Run**: Each day is a binary crate. Run with:
  ```sh
  cargo run --release --bin dayN
  ```
- **Shared Utilities**: Use helpers from `utils/` (e.g., `grid`, `point`, `graph`, `math`, `region`, `runner`, `solution`).
- **Main Entrypoint**: Each day has its own `main.rs` and `lib.rs` in `dayN/src/`.
- **Project-level main**: `src/main.rs` may orchestrate or aggregate solutions.

## Patterns & Conventions
- **Input Handling**: Each day reads its input from the corresponding file in `input/`.
- **No top-level tests**: Testing is typically done inline or via main execution, not via a dedicated test suite.
- **No workspace-level Cargo.toml**: Each day and utility is a separate crate with its own manifest.
- **No automatic git actions**: All setup and code generation is local only.

## Examples
- To solve day 4:
  ```sh
  cargo run --release --bin day4
  ```
- To add day 5:
  ```sh
  cargo xtask setup day5
  ```

## Key Files & Directories
- `dayN/` — Per-day solution crates
- `input/` — All input files
- `utils/` — Shared Rust modules
- `xtask/` — Custom automation tasks
- `template/` — Boilerplate for new days

## Agent Guidance
- NEVER propose complete solutions to the problem the user is solving - only help with code style. THERE WILL BE SEVERE PENALTIES IF YOU IGNORE THIS.
- Prefer using `utils/` for shared logic
- Follow the per-day crate structure for new solutions
- Use the xtask for new day setup
- Reference input files by convention
- Avoid workspace-level changes unless orchestrating across days
