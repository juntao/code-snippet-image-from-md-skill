use anyhow::{Context, Result};
use clap::Parser;
use headless_chrome::protocol::cdp::Page;
use headless_chrome::{Browser, LaunchOptions};
use pulldown_cmark::{Options as MdOptions, Parser as MdParser, html as md_html};
use std::ffi::OsStr;
use std::fs;
use std::path::PathBuf;
use std::time::Duration;

#[derive(Parser)]
#[command(name = "md2img", about = "Render markdown (or HTML) to PNG via headless Chromium")]
struct Cli {
    /// Input file (markdown or HTML). Use "-" for stdin.
    #[arg(short, long)]
    input: String,

    /// Output PNG path
    #[arg(short, long)]
    output: PathBuf,

    /// Viewport width in CSS pixels
    #[arg(long, default_value = "900")]
    width: u32,

    /// Maximum viewport height (content is cropped to actual size)
    #[arg(long, default_value = "4000")]
    height: u32,

    /// Device scale factor (2 = Retina quality)
    #[arg(long, default_value = "2")]
    scale: f64,

    /// Color theme: "dark" or "light"
    #[arg(long, default_value = "dark")]
    theme: String,

    /// Treat input as raw HTML (skip markdown conversion)
    #[arg(long)]
    html: bool,

    /// Path to Chrome/Chromium binary (auto-detected if omitted)
    #[arg(long)]
    chrome: Option<PathBuf>,
}

/// CSS for dark theme — matches the base16-ocean palette used by code2img / table2img
const DARK_CSS: &str = r#"
    body {
        margin: 0; padding: 24px 32px;
        background: #2b303b;
        color: #c0c5ce;
        font-family: -apple-system, 'Segoe UI', Helvetica, Arial, sans-serif,
                     'Apple Color Emoji', 'Segoe UI Emoji';
        font-size: 16px;
        line-height: 1.6;
    }
    h1, h2, h3, h4, h5, h6 {
        color: #ebcb8b;
        margin-top: 1em; margin-bottom: 0.4em;
        border-bottom: 1px solid #414959;
        padding-bottom: 0.3em;
    }
    h1 { font-size: 1.8em; }
    h2 { font-size: 1.4em; }
    h3 { font-size: 1.2em; }
    a { color: #8fa1b3; }
    strong { color: #d8dee9; }
    code {
        font-family: 'JetBrains Mono', 'SF Mono', 'Menlo', 'Consolas', monospace;
        background: #1e222a;
        padding: 2px 6px;
        border-radius: 4px;
        font-size: 0.9em;
    }
    pre {
        background: #1e222a;
        border: 1px solid #414959;
        border-radius: 6px;
        padding: 16px 20px;
        overflow-x: auto;
        line-height: 1.5;
    }
    pre code {
        background: none;
        padding: 0;
    }
    blockquote {
        border-left: 4px solid #ebcb8b;
        margin: 1em 0;
        padding: 0.5em 1em;
        background: #262b35;
        border-radius: 0 6px 6px 0;
    }
    table {
        border-collapse: collapse;
        border: 1px solid #414959;
        margin: 1em 0;
        width: auto;
    }
    th {
        background: #1e222a;
        color: #ebcb8b;
        padding: 10px 20px;
        border: 1px solid #414959;
        text-align: left;
        white-space: nowrap;
    }
    td {
        padding: 10px 20px;
        border: 1px solid #414959;
        white-space: nowrap;
    }
    tr:nth-child(even) td { background: #262b35; }
    tr:nth-child(odd) td  { background: #2b303b; }
    ul, ol { padding-left: 1.5em; }
    li { margin: 0.3em 0; }
    hr { border: none; border-top: 1px solid #414959; margin: 1.5em 0; }
    img { max-width: 100%; }
"#;

/// CSS for light theme
const LIGHT_CSS: &str = r#"
    body {
        margin: 0; padding: 24px 32px;
        background: #ffffff;
        color: #333333;
        font-family: -apple-system, 'Segoe UI', Helvetica, Arial, sans-serif,
                     'Apple Color Emoji', 'Segoe UI Emoji';
        font-size: 16px;
        line-height: 1.6;
    }
    h1, h2, h3, h4, h5, h6 {
        color: #1a1a1a;
        margin-top: 1em; margin-bottom: 0.4em;
        border-bottom: 1px solid #e0e0e0;
        padding-bottom: 0.3em;
    }
    h1 { font-size: 1.8em; }
    h2 { font-size: 1.4em; }
    h3 { font-size: 1.2em; }
    a { color: #0366d6; }
    code {
        font-family: 'JetBrains Mono', 'SF Mono', 'Menlo', 'Consolas', monospace;
        background: #f0f0f0;
        padding: 2px 6px;
        border-radius: 4px;
        font-size: 0.9em;
    }
    pre {
        background: #f6f8fa;
        border: 1px solid #e0e0e0;
        border-radius: 6px;
        padding: 16px 20px;
        overflow-x: auto;
        line-height: 1.5;
    }
    pre code { background: none; padding: 0; }
    blockquote {
        border-left: 4px solid #0366d6;
        margin: 1em 0;
        padding: 0.5em 1em;
        background: #f6f8fa;
        border-radius: 0 6px 6px 0;
    }
    table {
        border-collapse: collapse;
        border: 1px solid #d0d0d0;
        margin: 1em 0;
        width: auto;
    }
    th {
        background: #f0f0f0;
        color: #1a1a1a;
        padding: 10px 20px;
        border: 1px solid #d0d0d0;
        text-align: left;
        white-space: nowrap;
    }
    td {
        padding: 10px 20px;
        border: 1px solid #d0d0d0;
        white-space: nowrap;
    }
    tr:nth-child(even) td { background: #f8f8f8; }
    ul, ol { padding-left: 1.5em; }
    li { margin: 0.3em 0; }
    hr { border: none; border-top: 1px solid #e0e0e0; margin: 1.5em 0; }
    img { max-width: 100%; }
"#;

fn find_chrome() -> Option<PathBuf> {
    let home = std::env::var("HOME").ok()?;
    let home = PathBuf::from(home);

    // Playwright headless shell (preferred — lighter weight)
    let pw_dir = home.join("Library/Caches/ms-playwright");
    if pw_dir.exists() {
        if let Ok(entries) = fs::read_dir(&pw_dir) {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().to_string();
                if name.starts_with("chromium_headless_shell") || name.starts_with("chromium-") {
                    for variant in &[
                        "chrome-headless-shell-mac-arm64/chrome-headless-shell",
                        "chrome-headless-shell-mac-x64/chrome-headless-shell",
                        "chrome-headless-shell-linux-x64/chrome-headless-shell",
                        "chrome-headless-shell-linux-arm64/chrome-headless-shell",
                        "chrome-linux/chrome",
                        "chrome-mac-arm64/Chromium.app/Contents/MacOS/Chromium",
                        "chrome-mac/Chromium.app/Contents/MacOS/Chromium",
                    ] {
                        let bin = entry.path().join(variant);
                        if bin.exists() {
                            return Some(bin);
                        }
                    }
                }
            }
        }
    }

    // ~/.cache/ms-playwright (Linux default)
    let pw_linux = home.join(".cache/ms-playwright");
    if pw_linux.exists() {
        if let Ok(entries) = fs::read_dir(&pw_linux) {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().to_string();
                if name.starts_with("chromium_headless_shell") || name.starts_with("chromium-") {
                    for variant in &[
                        "chrome-headless-shell-linux-x64/chrome-headless-shell",
                        "chrome-headless-shell-linux-arm64/chrome-headless-shell",
                        "chrome-linux/chrome",
                    ] {
                        let bin = entry.path().join(variant);
                        if bin.exists() {
                            return Some(bin);
                        }
                    }
                }
            }
        }
    }

    // System Chrome / Chromium
    let candidates = [
        "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
        "/Applications/Chromium.app/Contents/MacOS/Chromium",
        "/usr/bin/google-chrome",
        "/usr/bin/google-chrome-stable",
        "/usr/bin/chromium",
        "/usr/bin/chromium-browser",
        "/snap/bin/chromium",
    ];
    for path in candidates {
        let p = PathBuf::from(path);
        if p.exists() {
            return Some(p);
        }
    }

    // Windows (common paths)
    #[cfg(target_os = "windows")]
    {
        let win_paths = [
            r"C:\Program Files\Google\Chrome\Application\chrome.exe",
            r"C:\Program Files (x86)\Google\Chrome\Application\chrome.exe",
        ];
        for path in win_paths {
            let p = PathBuf::from(path);
            if p.exists() {
                return Some(p);
            }
        }
    }

    None
}

fn markdown_to_html(md: &str, css: &str) -> String {
    let options = MdOptions::ENABLE_TABLES
        | MdOptions::ENABLE_STRIKETHROUGH
        | MdOptions::ENABLE_TASKLISTS;
    let parser = MdParser::new_ext(md, options);

    let mut body_html = String::new();
    md_html::push_html(&mut body_html, parser);

    format!(
        r#"<!DOCTYPE html>
<html>
<head>
<meta charset="UTF-8">
<style>{css}</style>
</head>
<body>
{body_html}
</body>
</html>"#
    )
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Read input
    let content = if cli.input == "-" {
        use std::io::Read;
        let mut buf = String::new();
        std::io::stdin().read_to_string(&mut buf)?;
        buf
    } else {
        fs::read_to_string(&cli.input)
            .with_context(|| format!("Failed to read input: {}", cli.input))?
    };

    let css = match cli.theme.as_str() {
        "light" => LIGHT_CSS,
        _ => DARK_CSS,
    };

    // Convert to HTML (or use as-is if --html)
    let html = if cli.html {
        content
    } else {
        markdown_to_html(&content, css)
    };

    // Write HTML to a temp file
    let temp_dir = std::env::temp_dir();
    let html_path = temp_dir.join("md2img_render.html");
    fs::write(&html_path, &html)?;

    // Find Chrome
    let chrome_path = cli.chrome
        .or_else(find_chrome)
        .context(
            "Could not find Chrome or Chromium.\n\n\
             md2img requires a Chromium-based browser for rendering.\n\
             Install one of:\n\
             • Playwright: pip install playwright && python -m playwright install chromium\n\
             • npx playwright install chromium\n\
             • Google Chrome: https://google.com/chrome\n\
             • Or specify a path: --chrome /path/to/chrome"
        )?;

    eprintln!("Using browser: {}", chrome_path.display());

    let scale_arg = format!("--force-device-scale-factor={}", cli.scale);
    let launch_options = LaunchOptions {
        path: Some(chrome_path),
        window_size: Some((cli.width, cli.height)),
        args: vec![
            OsStr::new("--disable-gpu"),
            OsStr::new("--no-sandbox"),
            OsStr::new("--disable-dev-shm-usage"),
            OsStr::new(Box::leak(scale_arg.into_boxed_str())),
        ],
        ..Default::default()
    };

    let browser = Browser::new(launch_options)
        .context("Failed to launch browser")?;

    let tab = browser.new_tab()
        .context("Failed to create tab")?;

    let file_url = format!("file://{}", html_path.display());
    tab.navigate_to(&file_url)?;
    tab.wait_until_navigated()?;

    // Allow rendering to settle
    std::thread::sleep(Duration::from_millis(300));

    // Measure actual content size
    let bounds = tab.evaluate(
        r#"
        (() => {
            const body = document.body;
            const rect = body.getBoundingClientRect();
            return JSON.stringify({
                width: Math.ceil(rect.width + 32),
                height: Math.ceil(rect.height + 32)
            });
        })()
        "#,
        false,
    )?;

    let bounds_str = bounds.value
        .as_ref()
        .and_then(|v| v.as_str())
        .context("Failed to measure content bounds")?;

    let bounds_json: serde_json::Value = serde_json::from_str(bounds_str)?;
    let clip_w = bounds_json["width"].as_f64().unwrap_or(cli.width as f64);
    let clip_h = bounds_json["height"].as_f64().unwrap_or(cli.height as f64);

    let screenshot_data = tab.capture_screenshot(
        Page::CaptureScreenshotFormatOption::Png,
        None,
        Some(Page::Viewport {
            x: 0.0,
            y: 0.0,
            width: clip_w,
            height: clip_h,
            scale: cli.scale,
        }),
        true,
    )?;

    fs::write(&cli.output, &screenshot_data)?;

    // Clean up temp file
    let _ = fs::remove_file(&html_path);

    println!("{}", cli.output.display());
    Ok(())
}
