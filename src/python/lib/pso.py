from typing import Callable, Iterator

import numpy as np

MultDimFn = Callable[[np.ndarray], float]


def pso_iter(
        func: MultDimFn, dim: int, n: int, s: float, a: float, b_loc: float, b_glob: float, c: float, d: float
) -> Iterator[np.ndarray]:
    shape = (n, dim)
    x = np.random.uniform(-s, s, shape)
    v = np.zeros(shape)
    p = x.copy()

    p_glob = p[np.argmin(np.apply_along_axis(func, 1, p))]

    while True:
        r_loc = np.random.random(shape)
        r_glob = np.random.random(shape)
        v = a * v + b_glob * r_glob * (p_glob - x) + b_loc * r_loc * (p - x)
        x = c * x + d * v

        for i in range(n):
            if func(x[i]) < func(p[i]):
                p[i] = x[i]
            if func(x[i] < func(p_glob)):
                p_glob = x[i]

        yield x
