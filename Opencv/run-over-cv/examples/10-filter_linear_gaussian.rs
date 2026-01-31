import cv2

img = cv2.imread('assets/lena.png')

#! 高斯滤波, 线性滤波方式
# 一种基于高斯函数的滤波处理, 它取的是卷积核区域内元素的加权均值, 用`cv2.GaussianBlur()`实现
# 参数3值越大, 模糊效果越明显.
img_gaussian = cv2.GaussianBlur(img, (3, 3), 1)
cv2.imwrite('assets/output/lena_filter_gaussian.png', img_gaussian)
