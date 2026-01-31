use opencv::core::{self, Mat, MatExprTraitConst, Vector};
use opencv::imgcodecs;
use opencv::imgproc;

fn main() -> Result<(), anyhow::Error> {
    // 需要指通颜色通道为 3 通道
    let mut img = Mat::zeros(300, 512, core::CV_8UC3)?.to_mat()?;
    // ! 画椭圆
    // 只需要定义椭圆的中心(x,y), x/y轴的长度, 以及angle旋转角度, startAngle椭圆的起始角度和endAngle椭圆的结束角度, 以及线的颜色和线宽即可.
    imgproc::ellipse(
        &mut img,
        core::Point { x: 256, y: 150 },
        core::Size {
            width: 200,
            height: 100,
        },
        0.0,
        0.0,
        360.0,
        core::Scalar::new(255.0, 0.0, 0.0, 0.0),
        -1,
        imgproc::LINE_8,
        0,
    )?;
    imgcodecs::imwrite("assets/output/draw_ellipse.png", &img, &Vector::new())?;
    Ok(())
}
