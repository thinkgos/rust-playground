use opencv::core::CV_VERSION;

// 验证opencv版本
fn main() -> Result<(), anyhow::Error> {
    println!("OpenCV version: {}", CV_VERSION);
    Ok(())
}
