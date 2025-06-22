# nof

https://user-images.githubusercontent.com/44772513/225290466-70da1118-8aee-4177-8d87-65c2d39150db.mp4

You can use it no matter which package manager you use: Yarn, npm, or pnpm

## Installation

To install nof, run the following command:

```bash
$ cargo install nof
```

## Usage

To use nof, run the following command:

```bash
Usage: nof <COMMAND>

Commands:
  completion       Generates a script for completion
  run              Run node scripts [aliases: r, R, run-script]
  install          Installs all dependencies [aliases: i, I]
  add              Installs a package [aliases: a, A]
  remove           remove a package [aliases: rm]
  execute-command  Run a command from a local or remote npm package [aliases: e, exec, E]
  help             Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version

```

This will launch fzf, which allows you to search for Node.js scripts. Once you've selected a script from the search results, it will be executed.

## Shell Completion Setup

NOF supports two types of shell completion:

### 1. Standard Completion (Basic)

```bash
# zsh
nof completion --shell=zsh > /usr/local/share/zsh/site-functions/_nof

# fish
nof completion --shell=fish > ~/.config/fish/completions/nof.fish

# bash
nof completion --shell=bash > /etc/bash_completion.d/nof
```

### 2. Dynamic Completion (Recommended)

Dynamic completion provides intelligent autocompletion for package.json scripts:

```bash
# zsh
nof completion --shell=zsh --custom > /usr/local/share/zsh/site-functions/_nof

# fish
nof completion --shell=fish --custom > ~/.config/fish/completions/nof.fish

# bash
nof completion --shell=bash --custom > /etc/bash_completion.d/nof
```

**Features of Dynamic Completion:**
- Auto-completes script names from package.json when typing `nof run <TAB>`
- Works with custom `--target-path` arguments
- Supports all run command aliases (`nof r <TAB>`, `nof R <TAB>`)
- Intelligently reads from the correct package.json file

**Example Usage:**
```bash
# After setup, you can use tab completion:
nof run <TAB>
# Shows: build  dev  test  start  lint  format  coverage  ...

nof run --target-path ../other-project/package.json <TAB>
# Shows scripts from the specified package.json
```
