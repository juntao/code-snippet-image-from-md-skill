---
name: code-snippet-images
description: Generate syntax-highlighted PNG images from code blocks in a markdown document. Triggered when user says they need images for code snippets in a markdown file. The markdown can be provided inline in the message body or as a Telegram file attachment. Generated images are sent back as Telegram messages. Supports dark (default) and light themes — user can request "light" or "dark" style.
---

# Code Snippet Images

Extract code blocks from markdown and render each as a syntax-highlighted PNG image.

## Binary

`{baseDir}/scripts/code2img` — Renders code to a syntax-highlighted PNG.

To rebuild from source:
```bash
cd {baseDir}/rust-src && cargo build --release
cp {baseDir}/rust-src/target/release/code2img {baseDir}/scripts/code2img
```

## Workflow

### 1. Extract Markdown

- **Inline**: Extract markdown text from the message body (everything after the request)
- **File attachment**: Read the file from the Telegram media path

### 2. Parse Code Blocks

Find all fenced code blocks (` ```lang ... ``` `). Extract each block's language, code content, and sequential index.

### 3. Render Images

For each code block, write code to a temp file and render:
```bash
{baseDir}/scripts/code2img -i /tmp/snippet_N.txt -o /tmp/snippet_N.png -l <lang>
```

Options:
- `-l <lang>` — Language for syntax highlighting (rust, python, js, etc.). Default: `plain`
- `--font-size <px>` — Font size. Default: `28`
- `--theme <name>` — Syntect theme. Default: `base16-ocean.dark`

**Style selection:** If the user requests "light" style/theme, use `--theme InspiredGitHub`. Default is dark (`base16-ocean.dark`). The user may also say "light mode", "white background", "light theme", etc.

### 4. Send Images via Telegram

Copy rendered PNGs to the allowed media directory, then send via the `message` tool:
```bash
cp /tmp/snippet_N.png ~/.openclaw/media/inbound/snippet_N.png
```

Then send:
- `action: send`
- `filePath: ~/.openclaw/media/inbound/snippet_N.png`
- `caption: "Code block N (lang)"`

Send images in order, one per code block. If no code blocks found, tell the user.
