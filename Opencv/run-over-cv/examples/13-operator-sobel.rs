use opencv::core::{self, Vector};
use opencv::imgcodecs;
use opencv::imgproc;
use opencv::prelude::*;

fn main() -> Result<(), anyhow::Error> {
    let img = imgcodecs::imread("assets/sun.png", imgcodecs::IMREAD_GRAYSCALE)?;
    // ! sobel算子
    // 水平
    let mut dst_x = Mat::default();
    imgproc::sobel_def(
        &img,         // 输入图片
        &mut dst_x,   // 输出图片
        core::CV_64F, // 深度
        1,            // 水平
        0,            // 垂直
    )?;
    let dst_x = core::abs(&dst_x)?;
    imgcodecs::imwrite(
        "assets/output/sun_operator_sobel_x.png",
        &dst_x,
        &Vector::new(),
    )?;

    // 垂直
    let mut dst_y = Mat::default();
    imgproc::sobel_def(
        &img,         // 输入图片
        &mut dst_y,   // 输出图片
        core::CV_64F, // 深度
        0,            // 水平
        1,            // 垂直
    )?;
    let dst_y = core::abs(&dst_y)?;
    imgcodecs::imwrite(
        "assets/output/sun_operator_sobel_y.png",
        &dst_y,
        &Vector::new(),
    )?;

    // 合并独立计算的水平/垂直的算子
    let mut dst_xy_merge = Mat::default();
    core::add_weighted_def(&dst_x, 0.5, &dst_y, 0.5, 0.0, &mut dst_xy_merge)?;
    imgcodecs::imwrite(
        "assets/output/sun_operator_sobel_xy_via_merge.png",
        &dst_xy_merge,
        &Vector::new(),
    )?;

    // 直接计算水平/垂直的算子
    let mut dst_direct = Mat::default();
    imgproc::sobel_def(
        &img,            // 输入图片
        &mut dst_direct, // 输出图片
        core::CV_64F,    // 深度
        1,               // 水平
        1,               // 垂直
    )?;
    let dst_direct = core::abs(&dst_direct)?;
    imgcodecs::imwrite(
        "assets/output/sun_operator_sobel_xy_via_direct.png",
        &dst_direct,
        &Vector::new(),
    )?;
    Ok(())
}
