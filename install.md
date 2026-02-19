# Images from Markdown Skill — Installation Guide

Follow these steps to install the skill that generates PNG images from code blocks, tables, and full markdown sections.

## Prerequisites

- `curl` or `wget` (for downloading)
- `unzip` (for extraction)
- `bash` (shell)
- `git` (for cloning)
- **For md2img only:** A Chromium-based browser (see below)

## Quick Install (Recommended)

```bash
SKILL_DIR="${HOME}/.openclaw/skills/images-from-md"
mkdir -p "$SKILL_DIR"

# Clone the skill
git clone --depth 1 https://github.com/juntao/images-from-md-skill.git /tmp/images-from-md-repo
cp -r /tmp/images-from-md-repo/* "$SKILL_DIR"
cp /tmp/images-from-md-repo/.gitignore "$SKILL_DIR" 2>/dev/null || true
rm -rf /tmp/images-from-md-repo

# Download platform-specific binaries
"${SKILL_DIR}/bootstrap.sh"
```

After installation, verify it works:

```bash
# Test code2img
echo 'print("hello")' > /tmp/test.py
~/.openclaw/skills/images-from-md/scripts/code2img -i /tmp/test.py -o /tmp/test_code.png -l python
ls -la /tmp/test_code.png

# Test table2img
printf '| Name | Value |\n|------|-------|\n| foo  | 42    |\n' > /tmp/test_table.md
~/.openclaw/skills/images-from-md/scripts/table2img -i /tmp/test_table.md -o /tmp/test_table.png
ls -la /tmp/test_table.png

# Test md2img (requires Chromium — see below)
printf '# Hello\n\nA table with emoji:\n\n| Status | Meaning |\n|--------|---------|\n| ✅ | Pass |\n| ❌ | Fail |\n' > /tmp/test.md
~/.openclaw/skills/images-from-md/scripts/md2img -i /tmp/test.md -o /tmp/test_md.png
ls -la /tmp/test_md.png
```

## Chromium Dependency (md2img only)

`md2img` renders markdown via headless Chromium. This gives it full support for emoji, Unicode, complex tables, and rich formatting. `code2img` and `table2img` do **not** require a browser.

If no browser is found, `md2img` will print a clear error message explaining how to install one.

### Option 1: Playwright (recommended)

Installs a lightweight headless Chromium shell (~90 MB):

```bash
pip install playwright && python -m playwright install chromium
# or
npx playwright install chromium
```

### Option 2: Google Chrome

Install from [google.com/chrome](https://google.com/chrome). `md2img` auto-detects Chrome in standard locations.

### Option 3: Specify manually

```bash
md2img -i input.md -o output.png --chrome /path/to/chrome
```

## Manual Binary Installation

If automatic download fails, manually download the binaries:

1. Go to https://github.com/juntao/images-from-md-skill/releases/latest
2. Download the zip for your platform:
   - `code-snippet-images-linux-x86_64.zip` (statically linked)
   - `code-snippet-images-linux-aarch64.zip` (statically linked)
   - `code-snippet-images-darwin-x86_64.zip`
   - `code-snippet-images-darwin-aarch64.zip`
   - `code-snippet-images-windows-x86_64.zip`
3. Extract to `~/.openclaw/skills/images-from-md/scripts/`
4. Make executable:
   ```bash
   chmod +x ~/.openclaw/skills/images-from-md/scripts/code2img
   chmod +x ~/.openclaw/skills/images-from-md/scripts/table2img
   chmod +x ~/.openclaw/skills/images-from-md/scripts/md2img
   ```

## Troubleshooting

### Download Failed
Check network connectivity:
```bash
curl -I "https://github.com/juntao/images-from-md-skill/releases/latest"
```

### md2img: "Could not find Chrome or Chromium"
Install a browser — see "Chromium Dependency" section above.

### Unsupported Platform
Check your platform:
```bash
echo "OS: $(uname -s), Arch: $(uname -m)"
```

Supported: Linux/macOS/Windows on x86_64/aarch64. Linux binaries are statically linked (musl) and work on any distro.
