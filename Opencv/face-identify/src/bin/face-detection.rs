use opencv::core;
use opencv::core::Scalar;
use opencv::imgproc;
use opencv::prelude::*;

use face_identify::yu_net::YuNet;

fn main() -> Result<(), anyhow::Error> {
    let mut detector = YuNet::new("../model/face_detection_yunet_2023mar.onnx")?;

    let mut image = opencv::imgcodecs::imread("assets/face1.png", opencv::imgcodecs::IMREAD_COLOR)?;

    detector.set_input_size(image.size()?)?;
    let (ret, faces) = detector.detect(&image)?;
    if ret == 0 {
        println!("No face detected.");
        return Ok(());
    }
    println!("Detected faces: {:?}", ret);

    // 获取一行检测结果
    let row = faces.at_row::<f32>(0)?;
    let (x, y, w, h) = (row[0], row[1], row[2], row[3]);

    imgproc::rectangle_def(
        &mut image,
        core::Rect::new(x as i32, y as i32, w as i32, h as i32),
        Scalar::new(0.0, 255.0, 0.0, 0.0),
    )?;

    opencv::imgcodecs::imwrite(
        "assets/face1-detect-result.png",
        &image,
        &core::Vector::default(),
    )?;

    Ok(())
}
