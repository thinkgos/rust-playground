import cv2

img = cv2.imread('assets/lena.png')

#! 方框滤波, 线性滤波方式
img_box = cv2.boxFilter(img, -1, (3, 3), normalize=True)
cv2.imwrite('assets/output/lena_filter_box.png', img_box)
