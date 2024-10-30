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

## setup auto completion

```bash
# zsh
nof completion --shell=zsh > /usr/local/share/zsh/site-functions/_nof
# fish
nof completion --shell=fish > ~/.config/fish/completions/nof.fish
```
```
