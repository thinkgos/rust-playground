use opencv::core::{self, Mat, MatTraitConst, Vector};
use opencv::{imgcodecs, imgproc};

fn main() -> Result<(), anyhow::Error> {
    let img = imgcodecs::imread("assets/lena.png", imgcodecs::IMREAD_COLOR)?;

    // ! 旋转
    // 同平移一样, 也是用仿射变换实现的. 因此需要定义一个变换矩阵.
    let rows = img.rows();
    let cols = img.cols();

    // # 参数1: 图片的旋转中心
    // # 参数2: 旋转角度(正: 逆时针, 负: 顺时针)
    // # 参数3: 缩放比例, 0.5 表示缩小一半
    let m = imgproc::get_rotation_matrix_2d(
        core::Point2f {
            x: rows as f32 / 2.0,
            y: cols as f32 / 2.0,
        },
        45.0,
        0.5,
    )?;

    let mut dst = Mat::default();

    imgproc::warp_affine(
        &img,
        &mut dst,
        &m,
        core::Size {
            width: cols,
            height: rows,
        },
        imgproc::INTER_LINEAR,
        core::BORDER_CONSTANT,
        core::Scalar::default(),
    )?;
    imgcodecs::imwrite(
        "assets/output/lena_geometry_rotate.png",
        &dst,
        &Vector::new(),
    )?;
    Ok(())
}
