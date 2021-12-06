import numpy as np


def read_population(filename: str) -> np.array:

    with open(filename) as file:
        data = [line.split(",") for line in file.readlines()]

    return np.array(data, dtype=np.int8).flatten()


def simulate_lanternfish(population: np.array, n_days: int = 80) -> np.array:

    for _ in range(n_days):
        population -= 1
        reproducing = population[population < 0]
        if len(reproducing) > 0:
            population[population < 0] = 6
            spawn = np.full((len(reproducing),), 8)
            population = np.append(population, spawn)

    return len(population)


if __name__ == "__main__":
    pop = read_population("d6.txt")

    part1 = simulate_lanternfish(pop)
    print(part1)
