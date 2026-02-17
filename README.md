# Code Snippet Images from Markdown

Generate syntax-highlighted PNG images from code blocks in markdown files. Built in Rust, designed for AI agents.

![Example output](https://img.shields.io/badge/rust-stable-orange) ![License: MIT](https://img.shields.io/badge/license-MIT-blue)

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
cd rust-src
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

## AI Agent Skill

This tool ships as an [OpenClaw](https://github.com/openclaw/openclaw) skill. AI agents can use it to generate code snippet images from markdown files on demand.

### Install as a Skill

Tell your agent:

> Read https://raw.githubusercontent.com/juntao/code-snippet-image-from-md-skill/main/install.md and follow the instructions.

Or install manually:

```bash
SKILL_DIR="${HOME}/.openclaw/skills/code-snippet-images"
mkdir -p "$SKILL_DIR"
git clone --depth 1 https://github.com/juntao/code-snippet-image-from-md-skill.git /tmp/csi-repo
cp -r /tmp/csi-repo/* "$SKILL_DIR"
rm -rf /tmp/csi-repo
"${SKILL_DIR}/bootstrap.sh"
```

## License

MIT
