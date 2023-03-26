# nof

https://user-images.githubusercontent.com/44772513/225290466-70da1118-8aee-4177-8d87-65c2d39150db.mp4

NOF is an open-source tool written in Rust that allows users to search and execute Node.js scripts using fzf. This tool is useful for developers and system administrators who want to efficiently execute Node.js scripts.
You can use it no matter which package manager you use: Yarn, npm, or pnpm
## Installation

To install RNSR, run the following command:
```bash
$ cargo install nof
```

## Usage

To use RNSR, run the following command:
```bash
Usage: nof <COMMAND>

Commands:
  run      Run node scripts [aliases: r, R, run-script]
  install  Installs all dependencies [aliases: i, I]
  add      Installs a package [aliases: a, A]
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version

```

This will launch fzf, which allows you to search for Node.js scripts. Once you've selected a script from the search results, it will be executed.


