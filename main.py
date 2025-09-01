import time

import cv2
import numpy as np
from matplotlib import pyplot as plt
from xdialog import open_file, save_file

image = open_file(
    "Select image to process",
    [
        (
            "Image files",
            " ".join(
                [
                    "*.bmp",
                    "*.dib",
                    "*.jpeg",
                    "*.jpe",
                    "*.jpg",
                    "*.jp2",
                    "*.png",
                    "*.webp",
                    "*.avif",
                    "*.pbm",
                    "*.pgm",
                    "*.ppm",
                    "*.pxm",
                    "*.pnm",
                    "*.pfm",
                    "*.sr",
                    "*.ras",
                    "*.tiff",
                    "*.tif",
                    "*.exr",
                    "*.hdr",
                    "*.pic",
                ]
            ),
        )
    ],
)
output = save_file("Save processed figure", [("PNG image", "*.png")])

start_time = time.time()


def print_time(action: str):
    global start_time

    new_time = time.time()
    print(f"{action} cost {int((new_time - start_time) * 1000)}ms")
    start_time = new_time


img = cv2.imread(image, 0)
print_time("Read")

f = np.fft.fft2(img)
print_time("FFT2")
fshift = np.fft.fftshift(f)
print_time("Shift")
magnitude_spectrum = 20 * np.log(np.abs(fshift))
print_time("Log")

plt.subplot(121), plt.imshow(img, cmap="gray")
plt.title("Input Image"), plt.xticks([]), plt.yticks([])
plt.subplot(122), plt.imshow(magnitude_spectrum, cmap="gray")
plt.title("Magnitude Spectrum"), plt.xticks([]), plt.yticks([])

print_time("Draw")

plt.savefig(output)
