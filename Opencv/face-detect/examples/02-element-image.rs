use opencv::core::Mat;
use opencv::core::Vector;
use opencv::imgcodecs;
use opencv::imgproc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let img = imgcodecs::imread("assets/lena.png", imgcodecs::IMREAD_ANYCOLOR)?;
    let mut dst = Mat::default();
    imgproc::cvt_color(&img, &mut dst, imgproc::COLOR_BGR2GRAY, 0)?;
    imgcodecs::imwrite("assets/output/lena_gray.png", &dst, &Vector::new())?;
    Ok(())
}
