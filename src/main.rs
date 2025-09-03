use std::error::Error;

use fft2d::{
    Complex,
    slice::{fft_2d, fftshift},
};
use image::{ImageBuffer, ImageFormat, ImageReader, Luma};
use palc::Parser;

use crate::args::CliArgs;

mod args;

const NORMALIZE: f64 = u16::MAX as f64;

fn main() -> Result<(), Box<dyn Error>> {
    let CliArgs {
        input_path,
        output_path,
    } = CliArgs::parse();
    let output_format = ImageFormat::from_extension(
        output_path
            .extension()
            .ok_or_else(|| "missing file extension of output path")?,
    )
    .ok_or_else(|| "unknown extension of output path")?;

    let gray_image = ImageReader::open(input_path)?
        .with_guessed_format()?
        .decode()?
        .into_luma16();
    let height = gray_image.height() as usize;
    let width = gray_image.width() as usize;

    let mut fft_buf = gray_image
        .as_raw()
        .iter()
        .map(|&pix| Complex::new(pix as f64 / NORMALIZE, 0.0))
        .collect::<Vec<_>>();
    fft_2d(width, height, &mut fft_buf);
    fft_buf = fftshift(width, height, &fft_buf);

    let img_buf = fft_buf.iter().map(|comp| (comp.re * NORMALIZE) as u16);
    let output_img =
        ImageBuffer::<Luma<u16>, Vec<u16>>::from_raw(width as _, height as _, img_buf.collect())
            .ok_or_else(|| "conversion error")?;

    output_img.save_with_format(output_path, output_format)?;

    Ok(())
}
