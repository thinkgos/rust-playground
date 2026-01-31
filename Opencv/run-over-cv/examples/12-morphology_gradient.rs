import cv2
import numpy as np

img = cv2.imread('assets/sun.png', cv2.IMREAD_GRAYSCALE)

# 形态学梯度 = 膨胀 - 腐蚀
kernel = np.ones((3, 3), np.uint8)
img_gradient = cv2.morphologyEx(img, cv2.MORPH_GRADIENT, kernel, iterations=1)
cv2.imwrite('assets/output/sun_morphology_gradient.png', img_gradient)
