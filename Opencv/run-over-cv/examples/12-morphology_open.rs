import cv2
import numpy as np

img = cv2.imread('assets/sun.png', cv2.IMREAD_GRAYSCALE)

# 开运算, 先腐蚀后膨胀
kernel = np.ones((3, 3), np.uint8)
img_open = cv2.morphologyEx(img, cv2.MORPH_OPEN, kernel, iterations=1)
cv2.imwrite('assets/output/sun_morphology_open.png', img_open)
