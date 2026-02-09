use opencv::objdetect::FaceDetectorYN;
use opencv::prelude::*;

fn main() -> Result<(), anyhow::Error> {
    // let mode = FaceDetectorYN::create(
    //     model,           // 模型路径
    //     config,          // 配置文件路径, ONNX 模型不需要配置文件
    //     input_size,      // 输入图像大小, (w, h)
    //     score_threshold, // 置信度阈值, 只有得分高于此值的结果才会被保留
    //     nms_threshold,   // 非极大值抑制（NMS）阈值（0.3）.用于过滤掉重复的检测框.
    //     top_k,           // 每一帧最多保留的前 $K$ 个检测结果
    //     backend_id,      // 推理后端
    //     target_id,       // 推理目标设备
    // )?;
    // mode.detect(image, faces);
    println!("Hello, world!");
    Ok(())
}
