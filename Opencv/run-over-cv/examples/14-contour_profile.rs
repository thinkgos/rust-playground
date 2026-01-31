import cv2

img = cv2.imread("assets/sun.png")
img_gray = cv2.cvtColor(img, cv2.COLOR_BGR2GRAY)
ret, thresh = cv2.threshold(img_gray, 0, 255, cv2.THRESH_BINARY_INV + cv2.THRESH_OTSU)

# 寻找二值化图中的轮廓
contours, hierarchy = cv2.findContours(
    thresh,
    cv2.RETR_TREE,
    cv2.CHAIN_APPROX_SIMPLE,
)
print(len(contours))

cnt = contours[0]

# 轮廓面积
area = cv2.contourArea(cnt)
# 轮廓周长
perimeter = cv2.arcLength(cnt, True)
print(area, perimeter)
# 图像矩
M = cv2.moments(cnt)
# 外接矩形
x, y, w, h = cv2.boundingRect(cnt)
# 最小外接圆
(x, y), radius = cv2.minEnclosingCircle(cnt)
# 拟合椭圆
ellipse = cv2.fitEllipse(cnt)
# 形状匹配
