use opencv::core::{self, Mat, MatExprTraitConst, Point, Vector};
use opencv::imgcodecs;
use opencv::imgproc;

fn main() -> Result<(), anyhow::Error> {
    // 需要指通颜色通道为 3 通道
    let mut img = Mat::zeros(300, 512, core::CV_8UC3)?.to_mat()?;
    // ! 画多边形
    // 定义四个顶点坐标
    let pts = Vector::from_slice(&[
        Point { x: 10, y: 50 },
        Point { x: 20, y: 100 },
        Point { x: 230, y: 230 },
        Point { x: 250, y: 40 },
    ]);
    // 顶点个数: 4, 矩阵变成 4*1*2 维
    // 第一个表示点的数量, 第二个表示通道, 第三个表示坐标
    let mut contours = Vector::<Vector<Point>>::new();
    contours.push(pts);
    // # 如果画多条直线, 可以一次性传多个2*1*2维的数组
    imgproc::polylines(
        &mut img,
        &contours,
        true,
        core::Scalar::new(0.0, 255.0, 255.0, 0.0),
        5,
        imgproc::LINE_8,
        0,
    )?;
    imgcodecs::imwrite("assets/output/draw_polygon.png", &img, &Vector::new())?;
    Ok(())
}
