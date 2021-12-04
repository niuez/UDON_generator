import numpy as np
from matplotlib import pyplot as plt
img = np.ones((100, 100), np.uint8) * 127
plt.imshow(img, cmap="gray")
plt.show()
