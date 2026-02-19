---
name: code-snippet-images
description: Generate syntax-highlighted PNG images from code blocks, tables, and/or full markdown sections in a document. Triggered when user says they need images for code snippets, tables, or markdown content. The markdown can be provided inline in the message body or as a Telegram file attachment. Generated images are sent back as Telegram messages. Supports dark (default) and light themes — user can request "light" or "dark" style.
---

# Code Snippet Images

Extract code blocks, tables, and markdown sections from documents and render each as a PNG image.

## Binaries

* `{baseDir}/scripts/code2img` — Renders code to a syntax-highlighted PNG.
* `{baseDir}/scripts/table2img` — Renders a markdown table to a styled PNG.
* `{baseDir}/scripts/md2img` — Renders any markdown (or HTML) to a PNG via headless Chromium. Supports emoji, Unicode, complex formatting, and everything a browser can render.

## When to Use Which Tool

- **`code2img`** — Best for individual code blocks. Produces pixel-perfect syntax highlighting with embedded JetBrains Mono font. No browser needed.
- **`table2img`** — Best for simple markdown tables without emoji or special Unicode. No browser needed.
- **`md2img`** — Use for:
  - Tables containing **emoji** (✅, ❌, ⚠️, etc.) — `table2img` cannot render emoji
  - **Mixed content** (headings + tables + code + blockquotes in one image)
  - Any markdown section the user wants rendered as a single image
  - When the user is **unsatisfied with existing output** from `code2img` or `table2img`
  - **Full markdown documents** or sections
  - Requires a Chromium-based browser (Playwright or Chrome)

## Workflow

### 1. Extract Markdown

- **Inline**: Extract markdown text from the message body (everything after the request)
- **File attachment**: Read the file from the Telegram media path

### 2. Parse Code Blocks and Tables

Find all fenced code blocks (` ```lang ... ``` `). Extract each block's language, code content, and sequential index.

### 3a. Render Code Images

For each code block, write code to a temp file and render:
```bash
{baseDir}/scripts/code2img -i /tmp/snippet_N.txt -o /tmp/snippet_N.png -l <lang>
```

Options:
- `-l <lang>` — Language for syntax highlighting (rust, python, js, etc.). Default: `plain`
- `--font-size <px>` — Font size. Default: `28`
- `--theme <name>` — Syntect theme. Default: `base16-ocean.dark`

**Style selection:** If the user requests "light" style/theme, use `--theme InspiredGitHub`. Default is dark (`base16-ocean.dark`). The user may also say "light mode", "white background", "light theme", etc.

### 3b. Render Table Images

For each markdown table, write the table (including header, separator, and data rows) to a temp file and render:
```bash
{baseDir}/scripts/table2img -i /tmp/table_N.md -o /tmp/table_N.png
```

Options:
- `--font-size <px>` — Font size. Default: `24`
- `--theme <dark|light>` — Color theme. Default: `dark`

**Style selection:** If the user requests "light" style/theme, use `--theme light`. Default is `dark`.

**Note:** If the table contains emoji or special Unicode characters, use `md2img` instead — `table2img` uses an embedded monospace font that lacks emoji glyphs.

### 3c. Render Markdown Sections (md2img)

For full markdown sections, mixed content, or when emoji rendering is needed:
```bash
{baseDir}/scripts/md2img -i /tmp/section_N.md -o /tmp/section_N.png
```

Options:
- `--width <px>` — Viewport width in CSS pixels. Default: `900`
- `--scale <factor>` — Device scale factor (2 = Retina). Default: `2`
- `--theme <dark|light>` — Color theme. Default: `dark`
- `--html` — Treat input as raw HTML (skip markdown conversion)
- `--chrome <path>` — Path to Chrome/Chromium binary (auto-detected if omitted)

**Style selection:** `--theme light` for light mode, `--theme dark` (default) for dark mode.

**Input from stdin:**
```bash
echo "# Hello 🦀" | {baseDir}/scripts/md2img -i - -o /tmp/output.png
```

### 4. Send Images via Telegram

Copy rendered PNGs to the allowed media directory, then send via the `message` tool:

**Code snippets:**
```bash
cp /tmp/snippet_N.png ~/.openclaw/media/inbound/snippet_N.png
```
Send with:
- `action: send`
- `filePath: ~/.openclaw/media/inbound/snippet_N.png`
- `caption: "Code block N (lang)"`

**Tables:**
```bash
cp /tmp/table_N.png ~/.openclaw/media/inbound/table_N.png
```
Send with:
- `action: send`
- `filePath: ~/.openclaw/media/inbound/table_N.png`
- `caption: "Table N"`

**Markdown sections:**
```bash
cp /tmp/section_N.png ~/.openclaw/media/inbound/section_N.png
```
Send with:
- `action: send`
- `filePath: ~/.openclaw/media/inbound/section_N.png`
- `caption: "Section N"`

Send all images in order (code blocks first, then tables, then sections). If no code blocks, tables, or sections found, tell the user.
