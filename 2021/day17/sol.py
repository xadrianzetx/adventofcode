from typing import Tuple

# target area: x=209..238, y=-86..-59
TAR_X_MAX = 238
TAR_X_MIN = 209
TAR_Y_MAX = -59
TAR_Y_MIN = -86


def simulate_trajectory(vx: int, vy: int) -> Tuple[int, bool]:

    x_pos, y_pos, max_y = 0, 0, 0
    while True:
        x_pos += vx
        y_pos += vy

        if y_pos > max_y:
            max_y = y_pos

        if TAR_X_MIN <= x_pos <= TAR_X_MAX and TAR_Y_MIN <= y_pos <= TAR_Y_MAX:
            # Landing zone reached.
            return max_y, True

        if x_pos > TAR_X_MAX or y_pos < TAR_Y_MIN:
            # Landing zone missed.
            return 0, False

        vy -= 1
        if vx > 0:
            vx -= 1
        elif vx < 0:
            vx += 1


def run_simulation() -> Tuple[int, int]:

    max_y, hits = 0, 0
    for x in range(300):
        for y in range(-300, 300):
            y, on_target = simulate_trajectory(x, y)
            if on_target:
                hits += 1
                if y > max_y:
                    max_y = y

    return max_y, hits


if __name__ == "__main__":
    part1, part2 = run_simulation()
    print(part1)
    print(part2)
