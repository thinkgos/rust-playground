import numpy as np
import cv2

#! 图像相加
x = np.uint8([250])
y = np.uint8([10])
img = cv2.add(x, y)
print(cv2.add(x, y))  # 250+10 = 260 => 255
print(x + y)  # 250+10 = 260 % 256 = 4

#! 图像混合
# dst=α×img1+β×img2+γ 其中γ是相当于一个修正值

img1 = cv2.imread('assets/dog1.jpeg')
img2 = cv2.imread('assets/dog2.jpeg')
print("img1 shape: ", img1.shape)
print("img2 shape: ", img2.shape)
# 图像混合需要形状一样
(rows, cols) = img1.shape[:2]
img2 = cv2.resize(img2, (cols, rows))
# 按权重进行混合
img = cv2.addWeighted(img1, 0.6, img2, 0.4, 0)
cv2.imwrite('assets/output/dog_mixture_blend.jpeg', img)