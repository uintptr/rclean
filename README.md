# rclean

Recursively clean build artifacts across multiple Rust projects at once.

`rclean` walks a directory tree, finds every Rust project (identified by a `Cargo.toml` next to a `target/` directory), and runs `cargo clean` on each one. Useful for reclaiming disk space when you have many projects under a common root.

## Installation

Requires [Rust and Cargo](https://rustup.rs/).

```sh
cargo install --git https://github.com/fsck/rclean
```

## Usage

```
rclean [OPTIONS]

Options:
  -d, --directory <DIRECTORY>  Root directory to search [default: current directory]
  -y, --yes                    Skip the confirmation prompt
  -v, --verbose                Print each project being cleaned
      --dry-run                Show what would be cleaned without running cargo clean
  -h, --help                   Print help
```

### Examples

Clean all Rust projects under the current directory:

```sh
rclean
```

Clean all Rust projects under a specific directory:

```sh
rclean --directory ~/projects
```

Preview what would be cleaned without making any changes:

```sh
rclean --dry-run --verbose
```

Skip the confirmation prompt (useful in scripts):

```sh
rclean --yes --directory ~/dev
```

## How it works

1. Walks the directory tree starting from `--directory`.
2. For each `target/` directory found, checks whether a `Cargo.toml` exists alongside it.
3. If so, runs `cargo clean` in that project directory.

The tool requires `cargo` to be available on `PATH`.
