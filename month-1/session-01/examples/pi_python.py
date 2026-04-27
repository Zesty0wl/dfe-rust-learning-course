"""Monte Carlo Pi estimator (Python version)

Run with:
    python pi_python.py

Expect ~35-60 seconds on a modern PC.
"""

import random
import time


def estimate_pi(n: int) -> float:
    inside = 0
    for _ in range(n):
        x = random.random()
        y = random.random()
        if x * x + y * y <= 1.0:
            inside += 1
    return 4.0 * inside / n


if __name__ == "__main__":
    n = 100_000_000
    start = time.time()
    result = estimate_pi(n)
    elapsed = time.time() - start
    print(f"Pi ≈ {result:.6f}")
    print(f"Time: {elapsed:.2f} seconds")
