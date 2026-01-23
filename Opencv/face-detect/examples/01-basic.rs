use opencv::core::CV_VERSION;

fn main() -> Result<(), anyhow::Error> {
    println!("OpenCV version: {}", CV_VERSION);
    Ok(())
}
