# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

NOF is a Rust CLI tool that provides unified Node.js script execution across different package managers (npm, yarn, pnpm, bun). It offers fuzzy finding capabilities using `skim` (fzf-like) for interactive script selection.

## Development Commands

```bash
# Build the project
cargo build

# Run tests
cargo test

# Run locally during development
cargo run -- run

# Install the tool
cargo install nof

# Code quality checks
cargo clippy

# Generate shell completions (standard)
cargo run -- completion --shell=zsh

# Generate custom completions with dynamic script support
cargo run -- completion --shell=zsh --custom
```

## Architecture

The codebase follows a modular design with clear separation:

- **`src/main.rs`**: CLI entry point using clap with subcommands (run, install, add, remove, etc.)
- **`src/fn_lib/`**: Core library modules
  - `package_commands.rs`: Command generation logic for different package managers
  - `match_package_maneger.rs`: Auto-detection of package manager type
  - `command_list.rs`: Static command configurations for npm/yarn/pnpm/bun
  - `run_command.rs`: Command execution and autocompletion
- **`src/fzf_scripts.rs`**: Interactive fuzzy finder implementation using skim
- **`src/read_package_json.rs`**: package.json parsing for script extraction
- **`src/completion.rs`**: Dynamic completion system for package.json scripts

## Package Manager Support

The tool automatically detects and supports:
- **npm**: Commands executed via `npx`
- **yarn**: Commands executed via `yarn -s run`
- **pnpm**: Commands executed via `pnpx`
- **bun**: Commands executed via `bun`

Package manager detection is based on lock files (package-lock.json, yarn.lock, pnpm-lock.yaml, bun.lockb).

## Testing

Test files are located in `test_files/` with different package manager scenarios in `test_files/package_test/`. Each subdirectory contains realistic package.json files for testing different package manager configurations.

## Dynamic Completion System

The tool includes a sophisticated completion system that provides dynamic autocompletion for package.json scripts:

- **`completion-scripts`**: Hidden command that outputs available scripts for the current directory
- **Custom completion scripts**: Generate shell-specific completion scripts with `--custom` flag
- **Script argument completion**: `nof run <TAB>` automatically completes script names from package.json
- **Target path awareness**: Completion respects `--target-path` argument for different package.json locations
- **Supported shells**: zsh, fish, bash with dynamic script completion
- **Fallback behavior**: Standard clap completion for unsupported shells

### Implementation Details

The dynamic completion system consists of:

1. **`src/completion.rs`**: Core completion module
   - `get_script_completions()`: Safely reads package.json and extracts script names
   - `generate_zsh_completion()`: Generates zsh completion script with dynamic script support
   - `generate_fish_completion()`: Generates fish shell completion script
   - `generate_bash_completion()`: Generates bash completion script
   - Error handling for missing/invalid package.json files

2. **Completion Commands**:
   - `nof completion --shell=<shell> --custom`: Generate custom completion scripts
   - `nof completion-scripts --target-path <path>`: Output available scripts (hidden command)

3. **Shell-Specific Features**:
   - **zsh**: Advanced argument parsing with `_arguments -C` and state handling
   - **fish**: Function-based completion with `__nof_complete_scripts`
   - **bash**: COMP_WORDS parsing with position-aware script completion

4. **Smart Path Detection**:
   - Automatically detects `--target-path` arguments in command line
   - Falls back to `./package.json` if no path specified
   - Handles both `--target-path=value` and `--target-path value` formats

### Setup Dynamic Completion

```bash
# zsh
nof completion --shell=zsh --custom > /usr/local/share/zsh/site-functions/_nof

# fish  
nof completion --shell=fish --custom > ~/.config/fish/completions/nof.fish

# bash
nof completion --shell=bash --custom > /etc/bash_completion.d/nof
```

### Usage Examples

After installing completion scripts, you can use dynamic completion:

```bash
# Basic completion - automatically suggests scripts from ./package.json
nof run <TAB>
# Output: build  dev  test  start  lint  format  coverage  e2e:all  ...

# With custom target path
nof run --target-path ../other-project/package.json <TAB>
# Output: scripts from the specified package.json

# Completion works with all run command aliases
nof r <TAB>         # Same as 'nof run <TAB>'
nof R <TAB>         # Same as 'nof run <TAB>'
nof run-script <TAB>  # Same as 'nof run <TAB>'

# Test completion functionality manually
nof completion-scripts --target-path ./test_files/aaa.json
# Output: build start test

# Generate different shell completions
nof completion --shell=zsh --custom    # Dynamic zsh completion
nof completion --shell=fish --custom   # Dynamic fish completion  
nof completion --shell=bash --custom   # Dynamic bash completion
```

### Features Added in Recent Implementation

1. **Dynamic Script Completion**: `nof run <TAB>` reads package.json and provides script name autocompletion
2. **Enhanced CLI Argument**: Script argument now includes help text: "Script name to run (auto-completed from package.json)"
3. **Multi-Shell Support**: Works with zsh, fish, and bash shells
4. **Path-Aware Completion**: Respects `--target-path` for different package.json locations
5. **Alias Support**: Completion works with all run command aliases (`r`, `R`, `run-script`)
6. **Error Handling**: Graceful handling of missing or invalid package.json files
7. **Position-Aware Logic**: Only completes script names at appropriate argument positions
8. **Comprehensive Testing**: Full test coverage for completion functionality

## Key Types

- `NodePackageMannegerType`: Enum defining supported package managers
- Scripts are parsed as `Vec<(String, String)>` tuples of (name, command)
- Commands support both direct execution and fuzzy finding modes