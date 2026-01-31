use opencv::core::{self, Mat, MatExprTraitConst, Vector};
use opencv::imgcodecs;
use opencv::imgproc;

fn main() -> Result<(), anyhow::Error> {
    // 需要指通颜色通道为 3 通道
    let mut img = Mat::zeros(300, 512, core::CV_8UC3)?.to_mat()?;
    // ! 画线
    // 只需要定义起始点和终止点, 以及线的颜色和线宽即可.
    imgproc::line(
        &mut img,
        core::Point { x: 0, y: 0 },
        core::Point { x: 512, y: 300 },
        core::Scalar::new(255.0, 0.0, 0.0, 0.0),
        5,
        imgproc::LINE_8,
        0,
    )?;
    imgcodecs::imwrite("assets/output/draw_line.png", &img, &Vector::new())?;
    Ok(())
}
