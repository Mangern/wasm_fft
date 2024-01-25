from PIL import Image
import numpy as np
import cv2

def imsh1000(img):
    return cv2.resize(img, (1000, int(1000 * img.shape[0] / img.shape[1])))

def scaled_no_aa(img):
    return img[::8, ::8]

img = Image.open("./chess.jpg")

a = np.asarray(img)[:,:, 0]
orig = np.asarray(img)[:,:,0]

A = np.fft.fft2(a)

A = np.fft.fftshift(A)

print(A.shape)

# Idk what the hell i am doing
N = 490
M = 650
A[:N, :] = 0
A[-N:, :] = 0
A[:, :M] = 0
A[:, -M:] = 0

A = np.fft.ifftshift(A)
a = np.fft.ifft2(A).real.clip(0, 255).astype(np.uint8)

b = scaled_no_aa(a)
c = cv2.resize(orig, (orig.shape[1] // 8, orig.shape[0] // 8))

cv2.imshow("Original", imsh1000(orig))
cv2.imshow("Uten aliasing", imsh1000(scaled_no_aa(orig)))
cv2.imshow("Med aliasing", imsh1000(b))
cv2.waitKey(0)

