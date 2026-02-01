use opencv::core::{self, Vector};
use opencv::imgcodecs;
use opencv::imgproc;
use opencv::prelude::*;

fn main() -> Result<(), anyhow::Error> {
    let img1 = imgcodecs::imread("assets/dog1.jpeg", imgcodecs::IMREAD_COLOR)?;
    let img2 = imgcodecs::imread("assets/dog2.jpeg", imgcodecs::IMREAD_COLOR)?;

    // ! 图像混合
    // 图像混合需要形状一样
    // dst=α×img1+β×img2+γ 其中γ是相当于一个修正值
    let img1_size = img1.size()?;
    let mut img2_resized = Mat::default();
    imgproc::resize(
        &img2,
        &mut img2_resized,
        img1_size,
        0.0,
        0.0,
        imgproc::INTER_LINEAR,
    )?;
    // 按权重进行混合
    let mut dst = Mat::default();
    core::add_weighted(
        &img1,         // 输入图片1
        0.6,           // 图片1的权重
        &img2_resized, // 输入图片2
        0.4,           // 图片2的权重
        0.0,           // 修正值
        &mut dst,      // 输出图片
        -1,            // 输出图片的深度
    )?;
    imgcodecs::imwrite("assets/output/dog_mixture_blend.jpeg", &dst, &Vector::new())?;

    Ok(())
}
