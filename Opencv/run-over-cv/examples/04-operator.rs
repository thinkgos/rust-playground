use opencv::core::{Rect, Vector};
use opencv::imgcodecs;
use opencv::prelude::*;

fn main() -> Result<(), anyhow::Error> {
    let img = imgcodecs::imread("assets/lena.png", imgcodecs::IMREAD_COLOR)?;

    let roi = img.roi(Rect {
        x: 120,
        y: 100,
        width: 130,
        height: 120,
    })?;
    imgcodecs::imwrite("assets/output/lena_operator_roi.png", &roi, &Vector::new())?;
    Ok(())
}
