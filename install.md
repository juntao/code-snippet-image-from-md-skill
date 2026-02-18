# Code Snippet Images Skill — Installation Guide

Follow these steps to install the skill that generates syntax-highlighted PNG images from code blocks in markdown files.

## Prerequisites

- `curl` or `wget` (for downloading)
- `unzip` (for extraction)
- `bash` (shell)
- `git` (for cloning)

## Quick Install (Recommended)

```bash
SKILL_DIR="${HOME}/.openclaw/skills/code-snippet-images"
mkdir -p "$SKILL_DIR"

# Clone the skill
git clone --depth 1 https://github.com/juntao/images-from-md-skill.git /tmp/code-snippet-images-repo
cp -r /tmp/code-snippet-images-repo/* "$SKILL_DIR"
cp /tmp/code-snippet-images-repo/.gitignore "$SKILL_DIR" 2>/dev/null || true
rm -rf /tmp/code-snippet-images-repo

# Download platform-specific binary
"${SKILL_DIR}/bootstrap.sh"
```

After installation, verify it works:

```bash
echo 'print("hello")' > /tmp/test.py
~/.openclaw/skills/code-snippet-images/scripts/code2img -i /tmp/test.py -o /tmp/test.png -l python
ls -la /tmp/test.png
```

## Manual Binary Installation

If automatic download fails, manually download the binary:

1. Go to https://github.com/juntao/images-from-md-skill/releases/latest
2. Download the zip for your platform:
   - `code-snippet-images-linux-x86_64.zip` (statically linked)
   - `code-snippet-images-linux-aarch64.zip` (statically linked)
   - `code-snippet-images-darwin-x86_64.zip`
   - `code-snippet-images-darwin-aarch64.zip`
   - `code-snippet-images-windows-x86_64.zip`
3. Extract to `~/.openclaw/skills/code-snippet-images/scripts/`
4. Make executable: `chmod +x ~/.openclaw/skills/code-snippet-images/scripts/code2img`

## Building from Source

If no pre-built binary is available for your platform:

```bash
cd ~/.openclaw/skills/code-snippet-images/code2img
cargo build --release
cp target/release/code2img ../scripts/code2img
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
