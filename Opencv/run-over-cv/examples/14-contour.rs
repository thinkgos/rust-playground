import cv2

img = cv2.imread("assets/sun.png")
img_gray = cv2.cvtColor(img, cv2.COLOR_BGR2GRAY)
ret, thresh = cv2.threshold(img_gray, 0, 255, cv2.THRESH_BINARY_INV + cv2.THRESH_OTSU)

# 寻找二值化图中的轮廓
contours, hierarchy = cv2.findContours(
    thresh,
    cv2.RETR_TREE,
    cv2.CHAIN_APPROX_SIMPLE,
)
print(len(contours))

# 绘制找出来的轮廓
cv2.drawContours(img, contours, -1, (0, 0, 255), 2)
img = cv2.imwrite("assets/sun_contour.png", img)
