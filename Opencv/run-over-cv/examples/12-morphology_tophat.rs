import cv2
import numpy as np

img = cv2.imread('assets/sun.png', cv2.IMREAD_GRAYSCALE)

# 形态学梯度 = 原图 - 开运算
kernel = np.ones((3, 3), np.uint8)
img_tophat = cv2.morphologyEx(img, cv2.MORPH_TOPHAT, kernel, iterations=1)
cv2.imwrite('assets/output/sun_morphology_tophat.png', img_tophat)
