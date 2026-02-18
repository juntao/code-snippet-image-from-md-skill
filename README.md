# Code Snippet Images from Markdown

Generate syntax-highlighted PNG images from code blocks in markdown files. Built in Rust, designed for AI agents.

![Example output](https://img.shields.io/badge/rust-stable-orange) ![License: MIT](https://img.shields.io/badge/license-MIT-blue)

## Quick Start

This tool ships as an [OpenClaw](https://github.com/openclaw/openclaw) skill. Install it by telling your AI agent:

> Read https://raw.githubusercontent.com/juntao/code-snippet-image-from-md-skill/main/install.md and follow the instructions.

Once installed, send your agent a markdown file and ask it to create images for the code snippets. It will extract each code block, render it as a syntax-highlighted PNG, and send the images back.

## What It Does

Give it a markdown file with fenced code blocks:

````markdown
```rust
fn main() {
    println!("Hello, world!");
}
```
````

Get back a beautiful syntax-highlighted PNG image — ready to embed in blog posts, tweets, presentations, or anywhere that doesn't support code formatting natively.

## Features

- **40+ languages** — Rust, Python, JavaScript, Go, C, bash, and more via [syntect](https://github.com/trishume/syntect)
- **JetBrains Mono** — Embedded monospace font for crisp rendering
- **Accurate spacing** — Uses real font metrics, no character width hacks
- **Multiple themes** — `base16-ocean.dark` (default), `Solarized (dark)`, `InspiredGitHub`, and more
- **Single binary** — No runtime dependencies, statically linked on Linux

## Installation

### Pre-built Binaries

Download from [Releases](https://github.com/juntao/code-snippet-image-from-md-skill/releases/latest):

| Platform | File |
|----------|------|
| Linux x86_64 (static) | `code-snippet-images-linux-x86_64.zip` |
| Linux aarch64 (static) | `code-snippet-images-linux-aarch64.zip` |
| macOS Intel | `code-snippet-images-darwin-x86_64.zip` |
| macOS Apple Silicon | `code-snippet-images-darwin-aarch64.zip` |
| Windows x86_64 | `code-snippet-images-windows-x86_64.zip` |

### Build from Source

```bash
cd code2img
cargo build --release
# Binary at: target/release/code2img
```

## Usage

```bash
# Basic usage
code2img -i code.rs -o output.png -l rust

# Read from stdin
cat code.py | code2img -i - -o output.png -l python

# Custom font size and theme
code2img -i code.js -o output.png -l javascript --font-size 24 --theme "Solarized (dark)"
```

### Options

| Flag | Default | Description |
|------|---------|-------------|
| `-i` | (required) | Input file path, or `-` for stdin |
| `-o` | (required) | Output PNG path |
| `-l` | `plain` | Language for syntax highlighting |
| `--font-size` | `28` | Font size in pixels |
| `--theme` | `base16-ocean.dark` | Color theme |

### Available Themes

- `base16-ocean.dark` (default)
- `base16-eighties.dark`
- `base16-mocha.dark`
- `InspiredGitHub`
- `Solarized (dark)`
- `Solarized (light)`
- `base16-ocean.light`

## License

MIT
