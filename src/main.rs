use clap::Parser;
use colored::Colorize;
use draw::{render::Renderer, Canvas, Drawing, Shape, Style, SvgRenderer, RGB};
use std::{
    fmt::{Debug, Display},
    fs,
};

use magick_rust::{FilterType, MagickWand};

const RESIZED_WIDTH: f64 = 256f64;
const NUMBER_COLORS: usize = 16;
const MAX_ITERATIONS: usize = 16;
const DRAW_SQUARE_WIDTH: u32 = 100;
const DRAW_PALLET_ROW_COUNT: u32 = 8;

fn main() {
    magick_rust::magick_wand_genesis();
    let cli = CLI::parse();

    let wand = MagickWand::new();
    wand.read_image(&cli.path).unwrap();
    let width = wand.get_image_width();
    let scale: f64 = RESIZED_WIDTH / width as f64;
    wand.scale_image(scale, scale, FilterType::Undefined)
        .unwrap();
    wand.kmeans(cli.num_colors, MAX_ITERATIONS, 0.0).unwrap();
    let pixels = wand.get_image_histogram().unwrap();
    let colors: Vec<_> = pixels
        .into_iter()
        .map(|pixel| {
            let red = pixel.get_red() * 255f64;
            let green = pixel.get_green() * 255f64;
            let blue = pixel.get_blue() * 255f64;

            RGBColor::new(red as u8, green as u8, blue as u8)
        })
        .collect();

    if cli.print_colors {
        print_pallet(&colors);
    } else {
        print_normal(&colors);
    }

    if let Some(output) = cli.output {
        let bytes: Vec<u8> = draw_pallet(&colors);
        fs::write(output, &bytes).unwrap();
    }
}

fn print_normal(pallet: &[RGBColor]) {
    for (i, color) in pallet.iter().enumerate() {
        if i == pallet.len() - 1 {
            print!("{color}");
        } else {
            println!("{color}");
        }
    }
}

fn print_pallet(pallet: &[RGBColor]) {
    for (i, color) in pallet.iter().enumerate() {
        print!("{}", color.get_color_display());
        if (i + 1) % 8 == 0 {
            println!();
        }
    }

    println!("\n\n");

    for color in pallet.iter() {
        println!("{} {}", color.get_color_display(), color.to_hex_code());
    }
}

fn draw_pallet(pallet: &[RGBColor]) -> Vec<u8> {
    let width = DRAW_PALLET_ROW_COUNT * DRAW_SQUARE_WIDTH;
    let height =
        (pallet.len() as f32 / DRAW_PALLET_ROW_COUNT as f32).ceil() as u32 * DRAW_SQUARE_WIDTH;

    // create a canvas to draw on
    let mut canvas = Canvas::new(width, height);

    for (i, color) in pallet.iter().enumerate() {
        let width = (i % 8) as u32 * DRAW_SQUARE_WIDTH;
        let height = (i / 8) as u32 * DRAW_SQUARE_WIDTH;
        // create a new drawing
        let rect = Drawing::new()
            // give it a shape
            .with_shape(Shape::Rectangle {
                width: DRAW_SQUARE_WIDTH,
                height: DRAW_SQUARE_WIDTH,
            })
            // move it around
            .with_xy(width as f32, height as f32)
            // give it a cool style
            .with_style(Style::filled(RGB::from(color)));

        // add it to the canvas
        canvas.display_list.add(rect);
    }

    // save the canvas as an svg
    SvgRenderer::new().render(&canvas)
}

struct RGBColor {
    red: u8,
    green: u8,
    blue: u8,
}

impl From<&RGBColor> for RGB {
    fn from(value: &RGBColor) -> Self {
        RGB::new(value.red, value.green, value.blue)
    }
}

impl RGBColor {
    fn new(red: u8, green: u8, blue: u8) -> RGBColor {
        RGBColor { red, green, blue }
    }

    fn to_hex_code(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.red, self.green, self.blue)
    }

    fn get_color_display(&self) -> String {
        "████"
            .truecolor(self.red, self.green, self.blue)
            .to_string()
    }
}

impl Debug for RGBColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_hex_code())
    }
}

impl Display for RGBColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_hex_code())
    }
}

#[derive(Parser)]
#[command(version = "0.1")]
#[command(about = "Image color pallet extraction")]
#[allow(clippy::upper_case_acronyms)]
struct CLI {
    #[arg(help = "path to input image")]
    path: String,

    #[arg(short, long, help = "output svg location")]
    output: Option<String>,

    #[arg(short, long, help = "weather to print pallet to terminal")]
    #[arg(default_value_t = false)]
    print_colors: bool,

    #[arg(short, long, help = "number of colors to extract")]
    #[arg(default_value_t = NUMBER_COLORS)]
    num_colors: usize,
}
