use clap::Parser;
use grid::Grid;
use image::imageops::FilterType;
use itertools::Itertools;

const BRIGHTNESS_ORDER: [char; 69] = [
    '.', '\'', '`', '^', '"', ',', ':', ';', 'I', 'l', '!', 'i', '>', '<', '~', '+', '_', '-', '?',
    ']', '[', '}', '{', '1', ')', '(', '|', '\\', '/', 't', 'f', 'j', 'r', 'x', 'n', 'u', 'v', 'c',
    'z', 'X', 'Y', 'U', 'J', 'C', 'L', 'Q', '0', 'O', 'Z', 'm', 'w', 'q', 'p', 'd', 'b', 'k', 'h',
    'a', 'o', '*', '#', 'M', 'W', '&', '8', '%', 'B', '@', '$',
];

#[derive(Debug, Parser)]
struct ConvertArgs {
    /// Path of the image file to be converted to ascii
    path: std::path::PathBuf,
    #[arg(long)]
    /// The max width in pixels (not characters) that the image should be resized to (maintaining aspect ratio)
    /// prior to conversion to ascii. If both width and height are omitted, the original dimensions will be used.
    width: Option<u32>,
    #[arg(long)]
    /// The max height in pixels (not characters) that the image should be resized to (if larger, maintaining aspect ratio)
    /// prior to conversion to ascii. If both width and height are omitted, the original dimensions will be used.
    height: Option<u32>,
}

fn main() {
    let args = ConvertArgs::parse();
    let mut image = image::open(std::path::Path::new(&args.path)).unwrap();
    let resize;
    let (width, height) = match (args.width, args.height) {
        (Some(w), Some(h)) => {
            resize = true;
            (w, h)
        }
        (Some(w), None) => {
            resize = true;
            (w, image.height())
        }
        (None, Some(h)) => {
            resize = true;
            (image.width(), h)
        }
        _ => {
            resize = false;
            (image.width(), image.height())
        }
    };

    if resize {
        image = image.resize(width, height, FilterType::Nearest);
    }
    let pixels: Vec<_> = image
        .to_luma8()
        .pixels()
        .map(|p| BRIGHTNESS_ORDER[p.0[0] as usize * BRIGHTNESS_ORDER.len() / 256])
        .collect();
    let grid = Grid::from_vec(pixels, width as usize);
    let rows = (0..grid.rows()).into_iter();
    let cols = (0..grid.cols()).into_iter();
    for (i, j) in rows.cartesian_product(cols) {
        print!("{}{}{}", grid[i][j], grid[i][j], grid[i][j]);
        if j == grid.cols() - 1 {
            println!();
        }
    }
}
