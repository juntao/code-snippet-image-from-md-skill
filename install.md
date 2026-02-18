# Images from Markdown Skill — Installation Guide

Follow these steps to install the skill that generates PNG images from code blocks and tables in markdown files.

## Prerequisites

- `curl` or `wget` (for downloading)
- `unzip` (for extraction)
- `bash` (shell)
- `git` (for cloning)

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
   ```

## Building from Source

If no pre-built binary is available for your platform:

```bash
# Build code2img
cd ~/.openclaw/skills/images-from-md/code2img
cargo build --release
cp target/release/code2img ../scripts/code2img

# Build table2img
cd ~/.openclaw/skills/images-from-md/table2img
cargo build --release
cp target/release/table2img ../scripts/table2img
```

## Troubleshooting

### Download Failed
Check network connectivity:
```bash
curl -I "https://github.com/juntao/images-from-md-skill/releases/latest"
```

### Unsupported Platform
Check your platform:
```bash
echo "OS: $(uname -s), Arch: $(uname -m)"
```

Supported: Linux/macOS/Windows on x86_64/aarch64. Linux binaries are statically linked (musl) and work on any distro.
