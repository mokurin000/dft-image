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

img = cv2.imread(image, 0)
f = np.fft.fft2(img)
fshift = np.fft.fftshift(f)
magnitude_spectrum = 20 * np.log(np.abs(fshift))

plt.subplot(121), plt.imshow(img, cmap="gray")
plt.title("Input Image"), plt.xticks([]), plt.yticks([])
plt.subplot(122), plt.imshow(magnitude_spectrum, cmap="gray")
plt.title("Magnitude Spectrum"), plt.xticks([]), plt.yticks([])
plt.savefig(save_file("Save processed figure", [("PNG image", "*.png")]))
