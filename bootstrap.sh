#!/bin/bash
# Bootstrap script for code-snippet-images skill
# Downloads and installs platform-specific code2img, table2img, and md2img binaries

set -e

REPO="juntao/images-from-md-skill"
SKILL_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SCRIPTS_DIR="${SKILL_DIR}/scripts"

detect_platform() {
    local os arch

    case "$(uname -s)" in
        Linux*)  os="linux" ;;
        Darwin*) os="darwin" ;;
        MINGW*|MSYS*|CYGWIN*) os="windows" ;;
        *)
            echo "Error: Unsupported operating system: $(uname -s)" >&2
            exit 1
            ;;
    esac

    case "$(uname -m)" in
        x86_64|amd64) arch="x86_64" ;;
        aarch64|arm64) arch="aarch64" ;;
        *)
            echo "Error: Unsupported architecture: $(uname -m)" >&2
            exit 1
            ;;
    esac

    echo "${os}-${arch}"
}

get_download_url() {
    local platform="$1"
    local artifact_name="code-snippet-images-${platform}.zip"
    local api_url="https://api.github.com/repos/${REPO}/releases/latest"
    local download_url

    if command -v curl &>/dev/null; then
        download_url=$(curl -sL "$api_url" | grep -o "https://github.com/${REPO}/releases/download/[^\"]*${artifact_name}" | head -1)
    elif command -v wget &>/dev/null; then
        download_url=$(wget -qO- "$api_url" | grep -o "https://github.com/${REPO}/releases/download/[^\"]*${artifact_name}" | head -1)
    else
        echo "Error: Neither curl nor wget found." >&2
        exit 1
    fi

    if [ -z "$download_url" ]; then
        echo "Error: Could not find release for platform ${platform}" >&2
        echo "Check https://github.com/${REPO}/releases for available downloads." >&2
        exit 1
    fi

    echo "$download_url"
}

download_binary() {
    local platform="$1"
    local url="$2"
    local temp_dir

    echo "Downloading code2img, table2img, and md2img for ${platform}..." >&2

    mkdir -p "${SCRIPTS_DIR}"

    temp_dir=$(mktemp -d)
    local zip_file="${temp_dir}/code-snippet-images-${platform}.zip"

    echo "Fetching from: ${url}" >&2
    if command -v curl &>/dev/null; then
        curl -sL -o "$zip_file" "$url"
    else
        wget -q -O "$zip_file" "$url"
    fi

    echo "Extracting binaries..." >&2
    if command -v unzip &>/dev/null; then
        unzip -q -o "$zip_file" -d "${SCRIPTS_DIR}"
    else
        echo "Error: unzip not found." >&2
        rm -rf "$temp_dir"
        exit 1
    fi

    if [[ "$(uname -s)" != MINGW* ]] && [[ "$(uname -s)" != MSYS* ]] && [[ "$(uname -s)" != CYGWIN* ]]; then
        chmod +x "${SCRIPTS_DIR}/code2img"
        chmod +x "${SCRIPTS_DIR}/table2img" 2>/dev/null || true
        chmod +x "${SCRIPTS_DIR}/md2img" 2>/dev/null || true
    fi

    rm -rf "$temp_dir"

    echo "Binaries installed to ${SCRIPTS_DIR}" >&2
}

check_chromium() {
    echo "" >&2
    echo "=== md2img: Chromium dependency check ===" >&2

    # Check Playwright installs
    local found=""
    for dir in \
        "$HOME/Library/Caches/ms-playwright" \
        "$HOME/.cache/ms-playwright"; do
        if [ -d "$dir" ]; then
            found="$dir"
            break
        fi
    done

    # Check system Chrome
    if [ -z "$found" ]; then
        for bin in \
            "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome" \
            "/usr/bin/google-chrome" \
            "/usr/bin/google-chrome-stable" \
            "/usr/bin/chromium" \
            "/usr/bin/chromium-browser" \
            "/snap/bin/chromium"; do
            if [ -x "$bin" ]; then
                found="$bin"
                break
            fi
        done
    fi

    if [ -n "$found" ]; then
        echo "✅ Chromium found: $found" >&2
    else
        echo "⚠️  md2img requires a Chromium-based browser for rendering." >&2
        echo "   Install one of the following:" >&2
        echo "" >&2
        echo "   # Option 1: Playwright (recommended — lightweight headless shell)" >&2
        echo "   pip install playwright && python -m playwright install chromium" >&2
        echo "   # or: npx playwright install chromium" >&2
        echo "" >&2
        echo "   # Option 2: Google Chrome" >&2
        echo "   https://google.com/chrome" >&2
        echo "" >&2
        echo "   md2img will not work until a browser is available." >&2
        echo "   code2img and table2img do NOT require a browser." >&2
    fi
}

main() {
    local platform
    platform=$(detect_platform)
    echo "Detected platform: ${platform}" >&2

    local download_url
    download_url=$(get_download_url "$platform")

    download_binary "$platform" "$download_url"

    check_chromium

    echo "" >&2
    echo "Installed:" >&2
    ls -1 "${SCRIPTS_DIR}" | grep -v '^\.' >&2
}

main "$@"
