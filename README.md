# Images from Markdown

Generate PNG images from code blocks, tables, and full markdown sections. Built in Rust, designed for AI agents.

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

### Full Markdown → Rich Images (NEW)

Give it any markdown content — headings, tables with emoji, code blocks, blockquotes, lists — and get back a beautifully rendered image:

```markdown
# Hardware Support

| GPU | Status |
|-----|--------|
| GTX 1060 | ✅ Works |
| RTX 3090 | ✅ Fast  |

> **Note:** Emoji and Unicode render perfectly.
```

`md2img` uses headless Chromium for rendering, so it supports everything a browser can: emoji, Unicode, complex layouts, and rich formatting.

## Tools

| Tool | Best For | Browser Required |
|------|----------|-----------------|
| `code2img` | Individual code blocks with syntax highlighting | No |
| `table2img` | Simple markdown tables (ASCII text only) | No |
| `md2img` | Full markdown, tables with emoji, mixed content, re-renders | **Yes** (Chromium) |

### When to use `md2img` over the others

- Tables with **emoji** (✅, ❌, ⚠️) — `table2img` can't render emoji (it uses an embedded monospace font)
- **Mixed content** — headings + tables + code + blockquotes in one image
- When the user is **unsatisfied** with `code2img` or `table2img` output
- **Any markdown** you want rendered as a single image

## Features

- **Three tools** — `code2img` for code, `table2img` for simple tables, `md2img` for everything else
- **40+ languages** — Rust, Python, JavaScript, Go, C, bash, and more via [syntect](https://github.com/trishume/syntect)
- **JetBrains Mono** — Embedded monospace font in `code2img` and `table2img`
- **Full emoji & Unicode** — `md2img` renders through a real browser engine
- **Dark & light themes** — All three tools support dark (default) and light themes
- **Retina output** — `md2img` renders at 2x scale by default for crisp images
- **Single binaries** — No runtime dependencies for `code2img`/`table2img`; `md2img` needs Chromium

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

Each zip contains `code2img`, `table2img`, and `md2img` binaries.

### Chromium for md2img

`md2img` requires a Chromium-based browser. Install one of:

```bash
# Option 1: Playwright (recommended — lightweight ~90MB headless shell)
pip install playwright && python -m playwright install chromium
# or: npx playwright install chromium

# Option 2: Google Chrome
# https://google.com/chrome — auto-detected in standard locations

# Option 3: Specify path directly
md2img -i input.md -o output.png --chrome /path/to/chrome
```

`code2img` and `table2img` do **not** require a browser.

### Build from Source

```bash
# Build code2img
cd code2img && cargo build --release

# Build table2img
cd ../table2img && cargo build --release

# Build md2img
cd ../md2img && cargo build --release
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

#### Available Themes (code2img)

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

### md2img

```bash
# Render markdown to PNG (dark theme, 2x Retina)
md2img -i document.md -o output.png

# Light theme
md2img -i document.md -o output.png --theme light

# Custom width and scale
md2img -i document.md -o output.png --width 1200 --scale 3

# Render raw HTML
md2img -i page.html -o output.png --html

# Read from stdin
echo "# Hello 🦀" | md2img -i - -o output.png

# Specify Chrome path
md2img -i document.md -o output.png --chrome /usr/bin/chromium
```

#### Options

| Flag | Default | Description |
|------|---------|-------------|
| `-i` | (required) | Input markdown (or HTML with `--html`), or `-` for stdin |
| `-o` | (required) | Output PNG path |
| `--width` | `900` | Viewport width in CSS pixels |
| `--height` | `4000` | Maximum viewport height (content is auto-cropped) |
| `--scale` | `2` | Device scale factor (2 = Retina) |
| `--theme` | `dark` | Color theme (`dark` or `light`) |
| `--html` | off | Treat input as raw HTML (skip markdown→HTML conversion) |
| `--chrome` | auto-detect | Path to Chrome/Chromium binary |

#### Browser Auto-Detection

`md2img` searches for a browser in this order:
1. Playwright headless shell (`~/Library/Caches/ms-playwright/` or `~/.cache/ms-playwright/`)
2. System Chrome/Chromium (standard install locations)
3. `--chrome` flag (manual override)

## License

MIT
