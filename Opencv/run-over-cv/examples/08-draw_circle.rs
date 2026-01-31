use opencv::core::{self, Mat, MatExprTraitConst, Vector};
use opencv::imgcodecs;
use opencv::imgproc;

fn main() -> Result<(), anyhow::Error> {
    // 需要指通颜色通道为 3 通道
    let mut img = Mat::zeros(300, 512, core::CV_8UC3)?.to_mat()?;
    // !画圆
    // 只需要定义圆的中心和半径, 以及线的颜色和线宽即可. 这里的线宽为-1表示填充圆.
    imgproc::circle(
        &mut img,
        core::Point { x: 256, y: 150 },
        100,
        core::Scalar::new(0.0, 0.0, 255.0, 0.0),
        -1,
        imgproc::LINE_8,
        0,
    )?;
    imgcodecs::imwrite("assets/output/draw_circle.png", &img, &Vector::new())?;
    Ok(())
}
