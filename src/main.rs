use core::f64;
use std::{error::Error, ops::Mul, path::Path};

use fft2d::{
    Complex,
    slice::{fft_2d, fftshift_zerocopy},
};
use image::{ImageBuffer, ImageFormat, ImageReader, Luma};
use palc::Parser;

use crate::args::CliArgs;

mod args;

type GrayWidth = u16;
const NORMALIZE: f64 = GrayWidth::MAX as f64;

fn fft_shift_image(
    factor: f64,
    input_path: impl AsRef<Path>,
    output_path: impl AsRef<Path>,
    skip_shift: bool,
) -> Result<(), Box<dyn Error>> {
    let output_format = ImageFormat::from_extension(
        output_path
            .as_ref()
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

    // transposed width/height
    let (width, height) = (height, width);

    if !skip_shift {
        unsafe {
            fftshift_zerocopy(width, height, &mut fft_buf);
        }
    }
    let scale = ((height * width) as f64).sqrt();
    let img_buf = fft_buf.iter().map(|comp| {
        (comp.re / scale)
            .mul(NORMALIZE)
            .abs()
            .log(f64::consts::E)
            .mul(
                factor
                    * match size_of::<GrayWidth>() {
                        1 => 1.0,
                        2 => 256.0,
                        _ => {
                            unreachable!("GrayWidth must be one of u8/u16")
                        }
                    },
            ) as _
    });
    let output_img = ImageBuffer::<Luma<GrayWidth>, Vec<GrayWidth>>::from_raw(
        width as _,
        height as _,
        img_buf.collect(),
    )
    .ok_or_else(|| "conversion error")?;

    output_img.save_with_format(output_path, output_format)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let CliArgs {
        factor,
        input_path,
        output_path,
        skip_shift,
    } = CliArgs::parse();

    fft_shift_image(factor, input_path, output_path, skip_shift)?;

    Ok(())
}
