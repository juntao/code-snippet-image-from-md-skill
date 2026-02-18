---
name: code-snippet-images
description: Generate syntax-highlighted PNG images from code blocks AND/OR tables in a markdown document. Triggered when user says they need images for code snippets or tables in a markdown file. The markdown can be provided inline in the message body or as a Telegram file attachment. Generated images are sent back as Telegram messages. Supports dark (default) and light themes — user can request "light" or "dark" style.
---

# Code Snippet Images

Extract code blocks and tables from markdown and render each as a PNG image.

## Binaries

* `{baseDir}/scripts/code2img` — Renders code to a syntax-highlighted PNG.
* `{baseDir}/scripts/table2img` — Renders a markdown table to a styled PNG.

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

Send all images in order (code blocks first, then tables). If no code blocks or tables found, tell the user.
