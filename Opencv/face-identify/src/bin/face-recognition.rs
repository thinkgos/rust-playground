use face_identify::sface::Sface;
use opencv::core::Vector;
use opencv::prelude::*;

use face_identify::yu_net::YuNet;

fn main() -> Result<(), anyhow::Error> {
    let mut detector = YuNet::new("../model/face_detection_yunet_2023mar.onnx")?;
    let mut recognizer = Sface::new("../model/face_recognition_sface_2021dec.onnx")?;

    let target = opencv::imgcodecs::imread("assets/face1.png", opencv::imgcodecs::IMREAD_COLOR)?;
    let query = opencv::imgcodecs::imread("assets/face1.png", opencv::imgcodecs::IMREAD_COLOR)?;

    detector.set_input_size(target.size()?)?;
    let (ret1, faces_target) = detector.detect(&target)?;
    if ret1 == 0 {
        println!("No target face detected.");
        return Ok(());
    }
    detector.set_input_size(query.size()?)?;
    let (ret2, faces_query) = detector.detect(&query)?;
    if ret2 == 0 {
        println!("No query face detected.");
        return Ok(());
    }
    println!("Detected query faces: {:?}, target face: {:?}", ret1, ret2);

    let face_target_row = faces_target.at_row::<f32>(0)?;
    let face_target_row = Vector::from_slice(&face_target_row[0..face_target_row.len()]);
    for i in 0..faces_query.rows() {
        let row = faces_query.at_row::<f32>(i)?;
        let row = Vector::from_slice(&row[0..row.len()]);
        let result = recognizer.r#match(&target, &row, &query, &face_target_row)?;
        println!("Match result: {:?}", result);
    }
    Ok(())
}
