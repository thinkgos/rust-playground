import cv2
import numpy as np

img = cv2.imread('assets/sun.png', cv2.IMREAD_GRAYSCALE)

# 黑帽 = 闭运算 - 原图
kernel = np.ones((3, 3), np.uint8)
img_blackhat = cv2.morphologyEx(img, cv2.MORPH_BLACKHAT, kernel, iterations=1)
cv2.imwrite('assets/output/sun_morphology_blackhat.png', img_blackhat)
