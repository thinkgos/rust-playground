use opencv::core;
use opencv::imgproc;
use opencv::objdetect::FaceDetectorYN;
use opencv::prelude::*;

fn main() -> Result<(), anyhow::Error> {
    let model = "../model/face_detection_yunet_2023mar.onnx";
    let config = "";
    let input_size = core::Size::new(320, 320);
    let score_threshold = 0.8;
    let nms_threshold = 0.3;
    let top_k = 5000;
    let backend_id = 0;
    let target_id = 0;

    let mut mode = FaceDetectorYN::create(
        model,           // 模型路径
        config,          // 配置文件路径, ONNX 模型不需要配置文件
        input_size,      // 输入图像大小, (w, h)
        score_threshold, // 置信度阈值, 只有得分高于此值的结果才会被保留
        nms_threshold,   // 非极大值抑制（NMS）阈值（0.3）.用于过滤掉重复的检测框.
        top_k,           // 每一帧最多保留的前 $K$ 个检测结果
        backend_id,      // 推理后端
        target_id,       // 推理目标设备
    )?;

    let mut image = opencv::imgcodecs::imread("人脸1.png", opencv::imgcodecs::IMREAD_COLOR)?;
    let mut faces = Mat::default();
    mode.set_input_size(image.size()?)?;
    let ret = mode.detect(&image, &mut faces)?;
    println!("{:?}", ret);

    let row = faces.at_row::<f32>(0)?;

    let (x, y, w, h) = (row[0], row[1], row[2], row[3]);

    imgproc::rectangle_def(
        &mut image,
        core::Rect {
            x: x as i32,
            y: y as i32,
            width: w as i32,
            height: h as i32,
        },
        core::Scalar::new(0.0, 255.0, 0.0, 0.0),
    )?;

    opencv::imgcodecs::imwrite("人脸1-detect-result.png", &image, &core::Vector::default())?;

    Ok(())
}
