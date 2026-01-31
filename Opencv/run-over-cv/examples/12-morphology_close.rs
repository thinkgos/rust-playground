import cv2
import numpy as np

img = cv2.imread('assets/sun.png', cv2.IMREAD_GRAYSCALE)

# 闭运算, 先膨胀后腐蚀
kernel = np.ones((3, 3), np.uint8)
img_close = cv2.morphologyEx(img, cv2.MORPH_CLOSE, kernel, iterations=1)
cv2.imwrite('assets/output/sun_morphology_close.png', img_close)
