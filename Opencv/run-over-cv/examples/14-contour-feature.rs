use opencv::core::{self, Vector};
use opencv::imgcodecs;
use opencv::imgproc;
use opencv::prelude::*;

fn main() -> Result<(), anyhow::Error> {
    let mut img_color = imgcodecs::imread("assets/sun_well.png", imgcodecs::IMREAD_COLOR)?;

    let mut img_gray = Mat::default();
    imgproc::cvt_color_def(&img_color, &mut img_gray, imgproc::COLOR_BGR2GRAY)?;

    // ! 轮廓特征
    let mut thresh = Mat::default();
    imgproc::threshold(
        &img_gray,                                         // 输入图片
        &mut thresh,                                       // 输出图片
        127.0,                                             // 阈值
        255.0,                                             // 最大值
        imgproc::THRESH_BINARY_INV + imgproc::THRESH_OTSU, // 类型
    )?;
    // 寻找二值化图中的轮廓
    let mut contours: Vector<Vector<core::Point>> = Vector::new();
    imgproc::find_contours_def(
        &thresh,            // 输入图片
        &mut contours,      // 输出图片
        imgproc::RETR_TREE, // 深度
        imgproc::CHAIN_APPROX_SIMPLE,
    )?;

    let cnt = contours.get(1)?;
    // 轮廓面积
    let area = imgproc::contour_area_def(&cnt)?;
    println!("area: {}", area);

    // 轮廓周长
    let perimeter = imgproc::arc_length(&cnt, true)?;
    println!("perimeter: {}", perimeter);

    // 图像矩 - 各类几何特征
    // https://en.wikipedia.org/wiki/Image_moment
    let m = imgproc::moments(&cnt, true)?;
    println!("m: {:?}", m);

    let mut hu_var = [0.0; 7];
    imgproc::hu_moments(m, &mut hu_var)?;
    println!("hu_var: {:?}", hu_var);

    // 外接矩形
    let rect = imgproc::bounding_rect(&cnt)?;
    println!("rect: {:?}", rect);

    // 最小外接矩形
    // 找到一个能包围物体的最小矩形
    let rect = imgproc::min_area_rect(&cnt)?;
    println!("min_area_rect: {:?}", rect);
    // 可以用

    let mut rect_points = core::Vec4f::default();
    imgproc::box_points(rect, &mut rect_points)?;
    println!("min_area_rect points: {:?}", rect_points);

     // 最小外接圆
    // 找到一个能包围物体的最小圆
    let mut center = core::Point2f::default();
    let mut radius = 0.0;
    imgproc::min_enclosing_circle(&cnt, &mut center, &mut radius)?;
    println!(
        "min_enclosing_circle: center: {:?}, radius: {}",
        center, radius
    );

    // 拟合椭圆
    let ellipse = imgproc::fit_ellipse(&cnt)?;
    println!("ellipse: {:?}", ellipse);
    imgproc::ellipse_rotated_rect_def(
        &mut img_color,
        ellipse,
        core::Scalar::new(0.0, 0.0, 255.0, 0.0),
    )?;
    imgcodecs::imwrite(
        "assets/output/sun_contour_feature.png",
        &img_color,
        &Vector::new(),
    )?;

    // 形状匹配, 检测两个形状之间的相似度, 值越小, 越相似
    // 形状匹配是通过图像矩(imgproc::hu_moments)来实现的
    // https://en.wikipedia.org/wiki/Image_moment#Rotation_invariant_moments
    let match_val = imgproc::match_shapes(
        &cnt,
        &cnt,
        imgproc::ShapeMatchModes::CONTOURS_MATCH_I1.into(),
        0.0,
    )?;
    println!("match_val: {}", match_val);

    Ok(())
}
