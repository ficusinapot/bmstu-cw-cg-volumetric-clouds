import matplotlib.pyplot as plt
from mpl_toolkits.mplot3d import Axes3D
import numpy as np
import ast

def parse_file(file_path):
    with open(file_path, 'r') as file:
        return np.array([ast.literal_eval(line.strip()) for line in file][:20000])

points = parse_file("./test.txt")

fig = plt.figure()
ax = fig.add_subplot(111, projection='3d')

x, y, z = points[:, 0], points[:, 1], points[:, 2]

ax.scatter(x, y, z, c='r', marker='o')
ax.set_xlabel('X')
ax.set_ylabel('Y')
ax.set_zlabel('Z')

plt.show()
