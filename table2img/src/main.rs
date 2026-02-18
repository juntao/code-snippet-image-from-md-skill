use ab_glyph::{Font, FontRef, PxScale, ScaleFont};
use clap::Parser;
use image::{Rgb, RgbImage};
use imageproc::drawing::{draw_filled_rect_mut, draw_line_segment_mut, draw_text_mut};
use imageproc::rect::Rect;
use std::fs;

#[derive(Parser)]
#[command(name = "table2img", about = "Render a markdown table to a PNG image")]
struct Cli {
    /// Input file containing a markdown table
    #[arg(short, long)]
    input: String,

    /// Output PNG path
    #[arg(short, long)]
    output: String,

    /// Font size in pixels
    #[arg(long, default_value = "24")]
    font_size: f32,

    /// Theme: "dark" or "light"
    #[arg(long, default_value = "dark")]
    theme: String,
}

struct Theme {
    bg: Rgb<u8>,
    header_bg: Rgb<u8>,
    alt_row_bg: Rgb<u8>,
    text: Rgb<u8>,
    header_text: Rgb<u8>,
    border: Rgb<u8>,
}

fn dark_theme() -> Theme {
    Theme {
        bg: Rgb([43, 48, 59]),
        header_bg: Rgb([30, 34, 42]),
        alt_row_bg: Rgb([38, 43, 53]),
        text: Rgb([192, 197, 206]),
        header_text: Rgb([235, 203, 139]),
        border: Rgb([65, 73, 89]),
    }
}

fn light_theme() -> Theme {
    Theme {
        bg: Rgb([255, 255, 255]),
        header_bg: Rgb([240, 240, 240]),
        alt_row_bg: Rgb([248, 248, 248]),
        text: Rgb([51, 51, 51]),
        header_text: Rgb([31, 31, 31]),
        border: Rgb([200, 200, 200]),
    }
}

fn parse_table(input: &str) -> (Vec<String>, Vec<Vec<String>>) {
    let lines: Vec<&str> = input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .collect();

    let mut headers = Vec::new();
    let mut rows = Vec::new();

    for (i, line) in lines.iter().enumerate() {
        // Skip separator lines (e.g. |---|---|)
        let stripped = line.replace('|', "").replace('-', "").replace(':', "").trim().to_string();
        if stripped.is_empty() {
            continue;
        }

        let cells: Vec<String> = line
            .trim_matches('|')
            .split('|')
            .map(|c| c.trim().to_string())
            .collect();

        if headers.is_empty() {
            headers = cells;
        } else {
            rows.push(cells);
        }
    }

    (headers, rows)
}

fn measure_text_width(text: &str, char_advance: f32) -> f32 {
    text.chars().count() as f32 * char_advance
}

fn main() {
    let cli = Cli::parse();

    let content = if cli.input == "-" {
        use std::io::Read;
        let mut buf = String::new();
        std::io::stdin().read_to_string(&mut buf).unwrap();
        buf
    } else {
        fs::read_to_string(&cli.input).expect("Failed to read input file")
    };

    let (headers, rows) = parse_table(&content);
    if headers.is_empty() {
        eprintln!("No table found in input");
        std::process::exit(1);
    }

    let theme = if cli.theme == "light" {
        light_theme()
    } else {
        dark_theme()
    };

    let font_data = include_bytes!("../assets/JetBrainsMono-Regular.ttf");
    let font = FontRef::try_from_slice(font_data).expect("Failed to load font");
    let scale = PxScale::from(cli.font_size);
    let scaled_font = font.as_scaled(scale);

    let glyph_id = font.glyph_id('M');
    let char_advance = scaled_font.h_advance(glyph_id);

    let num_cols = headers.len();
    let cell_pad_x: f32 = 20.0;
    let cell_pad_y: f32 = 12.0;
    let row_height = cli.font_size + cell_pad_y * 2.0;
    let outer_pad: f32 = 16.0;

    // Calculate column widths
    let mut col_widths: Vec<f32> = headers
        .iter()
        .map(|h| measure_text_width(h, char_advance))
        .collect();

    for row in &rows {
        for (j, cell) in row.iter().enumerate() {
            if j < col_widths.len() {
                let w = measure_text_width(cell, char_advance);
                if w > col_widths[j] {
                    col_widths[j] = w;
                }
            }
        }
    }

    // Add padding to each column
    for w in col_widths.iter_mut() {
        *w += cell_pad_x * 2.0;
    }

    let total_width = col_widths.iter().sum::<f32>() + outer_pad * 2.0;
    let total_rows = 1 + rows.len(); // header + data rows
    let total_height = total_rows as f32 * row_height + outer_pad * 2.0;

    let img_w = total_width.ceil() as u32;
    let img_h = total_height.ceil() as u32;

    let mut img = RgbImage::from_pixel(img_w, img_h, theme.bg);

    // Draw header background
    draw_filled_rect_mut(
        &mut img,
        Rect::at(outer_pad as i32, outer_pad as i32)
            .of_size(total_width as u32 - outer_pad as u32 * 2, row_height as u32),
        theme.header_bg,
    );

    // Draw alternating row backgrounds
    for i in 0..rows.len() {
        if i % 2 == 1 {
            let y = outer_pad + (i as f32 + 1.0) * row_height;
            draw_filled_rect_mut(
                &mut img,
                Rect::at(outer_pad as i32, y as i32)
                    .of_size(total_width as u32 - outer_pad as u32 * 2, row_height as u32),
                theme.alt_row_bg,
            );
        }
    }

    // Draw horizontal lines
    for i in 0..=total_rows {
        let y = outer_pad + i as f32 * row_height;
        draw_line_segment_mut(
            &mut img,
            (outer_pad, y),
            (total_width - outer_pad, y),
            theme.border,
        );
    }

    // Draw vertical lines
    let mut x = outer_pad;
    for i in 0..=num_cols {
        let top_y = outer_pad;
        let bot_y = outer_pad + total_rows as f32 * row_height;
        draw_line_segment_mut(&mut img, (x, top_y), (x, bot_y), theme.border);
        if i < num_cols {
            x += col_widths[i];
        }
    }

    // Draw header text
    let text_y_offset = cell_pad_y;
    let mut x = outer_pad;
    for (j, header) in headers.iter().enumerate() {
        let tx = x + cell_pad_x;
        let ty = outer_pad + text_y_offset;
        draw_text_mut(
            &mut img,
            theme.header_text,
            tx as i32,
            ty as i32,
            scale,
            &font,
            header,
        );
        if j < col_widths.len() {
            x += col_widths[j];
        }
    }

    // Draw data rows
    for (i, row) in rows.iter().enumerate() {
        let mut x = outer_pad;
        for (j, cell) in row.iter().enumerate() {
            let tx = x + cell_pad_x;
            let ty = outer_pad + (i as f32 + 1.0) * row_height + text_y_offset;
            draw_text_mut(
                &mut img,
                theme.text,
                tx as i32,
                ty as i32,
                scale,
                &font,
                cell,
            );
            if j < col_widths.len() {
                x += col_widths[j];
            }
        }
    }

    img.save(&cli.output).expect("Failed to save image");
    println!("{}", cli.output);
}
