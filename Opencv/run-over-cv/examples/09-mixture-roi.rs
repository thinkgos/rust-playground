use opencv::core::{self, Rect, Vector};
use opencv::imgcodecs;
use opencv::imgproc;
use opencv::prelude::*;

// #! 拉位操作
// img1 = cv2.imread('assets/lena.png')
// logo = cv2.imread('assets/logo.png')

// # 把log放在左上角, 只关心这一区域
// rows, cols = logo.shape[:2]

// # 创建掩膜
// logo2gray = cv2.cvtColor(logo,cv2.COLOR_BGR2GRAY)
// ret, mask = cv2.threshold(logo2gray, 10, 255, cv2.THRESH_BINARY)
// mask_inv = cv2.bitwise_not(mask)

// # 只对roi区域进行融合
// roi = img1[:rows, :cols]
// img1_bg = cv2.bitwise_and(roi, roi, mask=mask_inv)
// dst = cv2.add(img1_bg, logo)  # 进行融合
// img1[:rows, :cols] = dst  # 融合后放在原图上
// cv2.imwrite('assets/output/lena_mixture_logo.png', img1)
fn main() -> Result<(), anyhow::Error> {
    let img = imgcodecs::imread("assets/lena.png", imgcodecs::IMREAD_COLOR)?;
    let logo = imgcodecs::imread("assets/logo.png", imgcodecs::IMREAD_GRAYSCALE)?;

    // 把log放在左上角, 只关心这一区域
    let rows = logo.rows();
    let cols = logo.cols();

    // 创建掩膜
    let mut mask = Mat::default();
    imgproc::threshold(&logo, &mut mask, 10.0, 255.0, imgproc::THRESH_BINARY)?;
    let mut mask_inv = Mat::default();
    core::bitwise_not(&mask, &mut mask_inv, &core::no_array())?;
    let mut mask_inv_3ch = Mat::default();
    imgproc::cvt_color(&mask_inv, &mut mask_inv_3ch, imgproc::COLOR_GRAY2BGR, 0)?;

    // # 只对roi区域进行融合
    let roi = img.roi(Rect {
        x: 0,
        y: 0,
        width: rows,
        height: cols,
    })?;
    let mut roi_dst = Mat::default();
    core::bitwise_and(&roi, &mask_inv_3ch, &mut roi_dst, &core::no_array())?;
    let mut dest = Mat::default();
    core::add(&logo, &roi_dst, &mut dest, &core::no_array(), -1)?;

    imgcodecs::imwrite("assets/output/lena_mixture_logo.png", &dest, &Vector::new())?;
    Ok(())
}
