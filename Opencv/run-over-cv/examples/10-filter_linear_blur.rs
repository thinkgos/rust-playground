import cv2

img = cv2.imread('assets/lena.png')
#! 均值滤波, 线性滤波方式
# 一种最简单的滤波处理, 它取的是卷积核区域内元素的均值, 用`cv2.blur()`实现:
img_blur = cv2.blur(img, (3, 3))
cv2.imwrite('assets/output/lena_filter_blur.png', img_blur)