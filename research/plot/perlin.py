import matplotlib.pyplot as plt
import numpy as np
from scipy.interpolate import CubicSpline

x = np.array([0.0, 0.2, 1.0, 1.6, 2.0, 2.5, 3.0, 3.2, 4.0,4.5, 5.0])
y = np.array([0.0, 0.1, 0.0, -0.2, 0.0, 0.8, 0.0, -0.3,0.0, 0.5, 0.0])

x.sort()

cs = CubicSpline(x, y)

x_new = np.linspace(min(x), max(x), 500)
y_new = cs(x_new)

plt.plot(x_new, y_new, color='b')

# plt.plot(x, y, 'o', label='Исходные данные', color='r')

tangent_length = 0.15

for xi, yi in zip(np.array([0.0, 1.0, 2.0, 3.0, 4.0, 5.0]), np.array([0.0, 0.0, 0.0, 0.0, 0.0, 0.0])):
    slope = cs.derivative()(xi)

    x_tangent_start = xi - tangent_length
    x_tangent_end = xi + tangent_length

    # Вычисляем y-координаты для этих точек
    y_tangent_start = x_tangent_start * slope + (yi - xi * slope)
    y_tangent_end = x_tangent_end * slope + (yi - xi * slope)

    # Строим касательную линию между этими точками
    plt.plot([x_tangent_start, x_tangent_end], [y_tangent_start, y_tangent_end], color='orange')

# Оформление графика
plt.xlabel('x')
plt.ylabel('perlin(x)')
# plt.grid(True)
plt.show()
