use opencv::core::{Mat, Vector};
use opencv::imgcodecs;
use opencv::imgproc;

fn main() -> Result<(), anyhow::Error> {
    let img = imgcodecs::imread("assets/lena.png", imgcodecs::IMREAD_GRAYSCALE)?;

    // ! 固定阀值
    // 就是像素点值大于阈值变成一类值,小于阈值变成另一类值
    // 参数1: 图片, 一般是灰度图
    // 参数2: 设定的阀值
    // 参数3: 对于THRESH_BINARY、THRESH_BINARY_INV阈值方法所选用的最大阈值,一般为255
    // 参数4: 阀值方式, 有五种
    {
        let mut dst = Mat::default();
        imgproc::threshold(&img, &mut dst, 127.0, 255.0, imgproc::THRESH_BINARY)?;
        imgcodecs::imwrite(
            "assets/output/lena_threshold_binary.png",
            &dst,
            &Vector::new(),
        )?;
    }

    {
        let mut dst = Mat::default();
        imgproc::threshold(&img, &mut dst, 127.0, 255.0, imgproc::THRESH_BINARY_INV)?;
        imgcodecs::imwrite(
            "assets/output/lena_threshold_binary_inv.png",
            &dst,
            &Vector::new(),
        )?;
    }

    {
        let mut dst = Mat::default();
        imgproc::threshold(&img, &mut dst, 127.0, 255.0, imgproc::THRESH_TRUNC)?;
        imgcodecs::imwrite(
            "assets/output/lena_threshold_trunc.png",
            &dst,
            &Vector::new(),
        )?;
    }
    {
        let mut dst = Mat::default();
        imgproc::threshold(&img, &mut dst, 127.0, 255.0, imgproc::THRESH_TOZERO)?;
        imgcodecs::imwrite(
            "assets/output/lena_threshold_tozero.png",
            &dst,
            &Vector::new(),
        )?;
    }
    {
        let mut dst = Mat::default();
        imgproc::threshold(&img, &mut dst, 127.0, 255.0, imgproc::THRESH_TOZERO_INV)?;
        imgcodecs::imwrite(
            "assets/output/lena_threshold_tozero_inv.png",
            &dst,
            &Vector::new(),
        )?;
    }

    Ok(())
}
