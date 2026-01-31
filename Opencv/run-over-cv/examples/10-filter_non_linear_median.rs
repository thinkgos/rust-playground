import cv2

img = cv2.imread('assets/lena.png')

#! 中间滤波, 非线性滤波方式
# 取所有数排序后取中间的值. 非常适用于去除椒盐噪声和斑点噪声.
img_median = cv2.medianBlur(img, 3)
cv2.imwrite('assets/output/lena_filter_median.png', img_median)
