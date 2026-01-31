use opencv::core::{Rect, Vector};
use opencv::imgcodecs;
use opencv::prelude::*;
use run_over_cv::imgutil;

fn main() -> Result<(), anyhow::Error> {
    let img = imgcodecs::imread("assets/lena.png", imgcodecs::IMREAD_COLOR)?;
    println!("source shape: {:?}", imgutil::shape(&img));

    // 基本操作
    // 提取ROI区域
    let roi = img.roi(Rect {
        x: 120,
        y: 50,
        width: 250,
        height: 350,
    })?;
    imgcodecs::imwrite("assets/output/lena_operator_roi.png", &roi, &Vector::new())?;
    Ok(())
}
