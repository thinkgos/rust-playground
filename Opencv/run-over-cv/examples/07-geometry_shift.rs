use opencv::core::{self, Mat, MatTraitConst, Vector};
use opencv::{imgcodecs, imgproc};

fn main() -> Result<(), anyhow::Error> {
    let img = imgcodecs::imread("assets/lena.png", imgcodecs::IMREAD_COLOR)?;

    let rows = img.rows();
    let cols = img.cols();

    // !平移
    // 定义平移矩阵, 需要是 numpy 的 float32 类型
    // 平移矩阵, x 轴平移 100, y 轴平移 50
    let m: Mat = core::Mat::from_slice_2d(&[[1.0, 0.0, 100.0], [0.0, 1.0, 50.0]])?;

    let mut dst = Mat::default();
    // 用仿射变换实现平移
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
        "assets/output/lena_geometry_shift.png",
        &dst,
        &Vector::new(),
    )?;
    Ok(())
}
