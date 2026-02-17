use ab_glyph::{Font, FontRef, PxScale, ScaleFont};
use clap::Parser;
use image::{Rgb, RgbImage};
use imageproc::drawing::draw_text_mut;
use std::fs;
use syntect::easy::HighlightLines;
use syntect::highlighting::{Style, ThemeSet};
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;

#[derive(Parser)]
#[command(name = "code2img", about = "Render a code block to a PNG image")]
struct Cli {
    /// Input file containing code (or - for stdin)
    #[arg(short, long)]
    input: String,

    /// Output PNG path
    #[arg(short, long)]
    output: String,

    /// Language for syntax highlighting (e.g. rust, python, javascript)
    #[arg(short, long, default_value = "plain")]
    lang: String,

    /// Font size in pixels
    #[arg(long, default_value = "28")]
    font_size: f32,

    /// Theme name (e.g. base16-ocean.dark, Solarized (dark), InspiredGitHub)
    #[arg(long, default_value = "base16-ocean.dark")]
    theme: String,
}

fn main() {
    let cli = Cli::parse();

    let code = if cli.input == "-" {
        use std::io::Read;
        let mut buf = String::new();
        std::io::stdin().read_to_string(&mut buf).unwrap();
        buf
    } else {
        fs::read_to_string(&cli.input).expect("Failed to read input file")
    };

    let ss = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();

    let theme = ts.themes.get(&cli.theme).unwrap_or_else(|| {
        eprintln!(
            "Theme '{}' not found. Available: {:?}",
            cli.theme,
            ts.themes.keys().collect::<Vec<_>>()
        );
        std::process::exit(1);
    });

    let syntax = ss
        .find_syntax_by_token(&cli.lang)
        .unwrap_or_else(|| ss.find_syntax_plain_text());

    let mut h = HighlightLines::new(syntax, theme);

    // Collect highlighted lines
    let mut highlighted_lines: Vec<Vec<(Style, String)>> = Vec::new();
    for line in LinesWithEndings::from(&code) {
        let ranges = h.highlight_line(line, &ss).unwrap();
        let styled: Vec<(Style, String)> = ranges
            .into_iter()
            .map(|(style, text)| (style, text.to_string()))
            .collect();
        highlighted_lines.push(styled);
    }

    // Use embedded font (monospace)
    let font_data = include_bytes!("../assets/JetBrainsMono-Regular.ttf");
    let font = FontRef::try_from_slice(font_data).expect("Failed to load font");
    let scale = PxScale::from(cli.font_size);
    let scaled_font = font.as_scaled(scale);

    // Get the actual advance width of a monospace character using 'M'
    let glyph_id = font.glyph_id('M');
    let char_advance = scaled_font.h_advance(glyph_id);

    // Calculate image dimensions
    let line_height = (cli.font_size * 1.5) as u32;
    let padding = 32u32;
    let max_line_len = code.lines().map(|l| l.len()).max().unwrap_or(0);
    let img_width = (max_line_len as f32 * char_advance + padding as f32 * 2.0).max(400.0) as u32;
    let img_height = (highlighted_lines.len() as u32 * line_height + padding * 2).max(100);

    // Background color from theme
    let bg = theme.settings.background.unwrap_or(syntect::highlighting::Color {
        r: 43,
        g: 48,
        b: 59,
        a: 255,
    });
    let mut img = RgbImage::from_pixel(img_width, img_height, Rgb([bg.r, bg.g, bg.b]));

    // Draw each line with syntax colors
    for (line_idx, spans) in highlighted_lines.iter().enumerate() {
        let y = padding as i32 + (line_idx as i32 * line_height as i32);
        let mut x_offset: f32 = padding as f32;

        for (style, text) in spans {
            let color = Rgb([style.foreground.r, style.foreground.g, style.foreground.b]);
            let draw_text_str = text.trim_end_matches('\n').trim_end_matches('\r');
            if !draw_text_str.is_empty() {
                draw_text_mut(&mut img, color, x_offset as i32, y, scale, &font, draw_text_str);
                // Advance by actual glyph width * character count
                x_offset += draw_text_str.chars().count() as f32 * char_advance;
            }
        }
    }

    img.save(&cli.output).expect("Failed to save image");
    println!("{}", cli.output);
}
