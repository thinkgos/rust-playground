use std::ops::{Deref, DerefMut};

use opencv::prelude::*;
use opencv::{
    core::{Mat, Ptr, Size, ToInputArray},
    objdetect::FaceDetectorYN,
};

pub struct YuNet {
    mode: Ptr<FaceDetectorYN>,
}

impl YuNet {
    pub fn new(path: &str) -> Result<Self, anyhow::Error> {
        let mode = FaceDetectorYN::create(
            path,                // 模型路径
            "",                  // 配置文件路径, ONNX 模型不需要配置文件
            Size::new(320, 320), // 输入图像大小, (w, h)
            0.8,                 // 置信度阈值, 只有得分高于此值的结果才会被保留
            0.3,                 // 非极大值抑制（NMS）阈值（0.3）.用于过滤掉重复的检测框.
            5000,                // 每一帧最多保留的前 $K$ 个检测结果
            0,                   // 推理后端
            0,                   // 推理目标设备
        )?;
        Ok(Self { mode })
    }
    pub fn detect(&mut self, image: &impl ToInputArray) -> Result<(i32, Mat), anyhow::Error> {
        let mut faces = Mat::default();
        let ret = self.mode.detect(image, &mut faces)?;
        Ok((ret, faces))
    }
}

impl Deref for YuNet {
    type Target = Ptr<FaceDetectorYN>;

    fn deref(&self) -> &Self::Target {
        &self.mode
    }
}

impl DerefMut for YuNet {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.mode
    }
}
