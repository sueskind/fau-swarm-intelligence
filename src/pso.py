from p5 import *

from lib.pso import pso_iter

s = 100


def square(xi: np.ndarray) -> float:
    if np.any(xi < -s) or np.any(xi > s):
        return np.inf
    return float(np.sum(xi ** 2))


iterator = pso_iter(square, dim=2, n=20, s=100, a=0.9, b_loc=0.3, b_glob=0.3, c=1, d=1)


def setup():
    size(700, 700)
    stroke_weight(5)
    stroke(0)
    fill(0)


def draw():
    background(255)
    translate(width / 2, height / 2)

    x = next(iterator)
    for xi in x:
        point(*xi)

    time.sleep(0.01)


if __name__ == "__main__":
    run()
