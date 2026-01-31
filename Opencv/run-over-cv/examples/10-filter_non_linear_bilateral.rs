import cv2

img = cv2.imread('assets/lena.png')

#! 双边滤波, 非线性滤波方式
# 图像的边缘信息能有效的保留下来
img_bilateral = cv2.bilateralFilter(img, 9, 75, 75)
cv2.imwrite('assets/output/lena_filter_bilateral.png', img_bilateral)