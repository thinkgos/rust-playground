import cv2
import numpy as np

img = cv2.imread('assets/sun.png', cv2.IMREAD_GRAYSCALE)

kernel = np.ones((3, 3), np.uint8)
img_erode = cv2.erode(img, kernel, iterations=1)
cv2.imwrite('assets/output/sun_morphology_erode.png', img_erode)
