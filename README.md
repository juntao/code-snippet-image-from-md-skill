# Images from Markdown

Generate PNG images from code blocks and tables in markdown files. Built in Rust, designed for AI agents.

![Example output](https://img.shields.io/badge/rust-stable-orange) ![License: MIT](https://img.shields.io/badge/license-MIT-blue)

## Quick Start

This tool ships as an [OpenClaw](https://github.com/openclaw/openclaw) skill. Install it by telling your AI agent:

> Read https://raw.githubusercontent.com/juntao/images-from-md-skill/main/install.md and follow the instructions.

Once installed, send your agent a markdown file and ask it to create images for the code snippets and tables. It will extract each code block and table, render them as PNG images, and send the images back.

## What It Does

### Code Blocks → Syntax-Highlighted Images

Give it a markdown file with fenced code blocks:

````markdown
```rust
fn main() {
    println!("Hello, world!");
}
```
````

Get back a beautiful syntax-highlighted PNG image — ready to embed in blog posts, tweets, presentations, or anywhere that doesn't support code formatting natively.

### Tables → Styled Table Images

Give it a markdown table:

```markdown
| Feature | Status |
|---------|--------|
| Code    | ✓      |
| Tables  | ✓      |
```

Get back a clean, styled table image with gridlines, header highlighting, and alternating row colors.

## Features

- **Two tools** — `code2img` for code blocks, `table2img` for markdown tables
- **40+ languages** — Rust, Python, JavaScript, Go, C, bash, and more via [syntect](https://github.com/trishume/syntect)
- **JetBrains Mono** — Embedded monospace font for crisp rendering
- **Accurate spacing** — Uses real font metrics, no character width hacks
- **Dark & light themes** — Both tools support dark (default) and light themes
- **Single binaries** — No runtime dependencies, statically linked on Linux

## Installation

### Pre-built Binaries

Download from [Releases](https://github.com/juntao/images-from-md-skill/releases/latest):

| Platform | File |
|----------|------|
| Linux x86_64 (static) | `code-snippet-images-linux-x86_64.zip` |
| Linux aarch64 (static) | `code-snippet-images-linux-aarch64.zip` |
| macOS Intel | `code-snippet-images-darwin-x86_64.zip` |
| macOS Apple Silicon | `code-snippet-images-darwin-aarch64.zip` |
| Windows x86_64 | `code-snippet-images-windows-x86_64.zip` |

Each zip contains both `code2img` and `table2img` binaries.

### Build from Source

```bash
# Build code2img
cd code2img
cargo build --release
# Binary at: target/release/code2img

# Build table2img
cd ../table2img
cargo build --release
# Binary at: target/release/table2img
```

## Usage

### code2img

```bash
# Basic usage
code2img -i code.rs -o output.png -l rust

# Read from stdin
cat code.py | code2img -i - -o output.png -l python

# Custom font size and theme
code2img -i code.js -o output.png -l javascript --font-size 24 --theme "Solarized (dark)"
```

#### Options

| Flag | Default | Description |
|------|---------|-------------|
| `-i` | (required) | Input file path, or `-` for stdin |
| `-o` | (required) | Output PNG path |
| `-l` | `plain` | Language for syntax highlighting |
| `--font-size` | `28` | Font size in pixels |
| `--theme` | `base16-ocean.dark` | Color theme |

#### Available Themes

- `base16-ocean.dark` (default)
- `base16-eighties.dark`
- `base16-mocha.dark`
- `InspiredGitHub`
- `Solarized (dark)`
- `Solarized (light)`
- `base16-ocean.light`

### table2img

```bash
# Basic usage (dark theme)
table2img -i table.md -o output.png

# Light theme
table2img -i table.md -o output.png --theme light

# Custom font size
table2img -i table.md -o output.png --font-size 20
```

#### Options

| Flag | Default | Description |
|------|---------|-------------|
| `-i` | (required) | Input file containing a markdown table, or `-` for stdin |
| `-o` | (required) | Output PNG path |
| `--font-size` | `24` | Font size in pixels |
| `--theme` | `dark` | Color theme (`dark` or `light`) |

## License

MIT
