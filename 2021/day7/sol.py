from typing import Callable

import numpy as np
import optuna


def read_crab_positions(filename: str) -> np.array:

    with open(filename) as file:
        data = [line.rstrip().split(",") for line in file.readlines()]
    return np.array([d for d in data[0]], dtype=np.int32)


def create_objective(part2: bool) -> Callable:
    def _objective(trial: optuna.Trial) -> float:

        pos = trial.suggest_int("pos", crabs.min(), crabs.max())
        distances = np.abs(crabs - pos)
        if part2:
            distances = distances * (distances + 1) / 2
        return np.sum(distances)

    return _objective


if __name__ == "__main__":
    crabs = read_crab_positions("d7.txt")
    optuna.logging.disable_default_handler()

    study_pt1 = optuna.create_study()
    study_pt1.optimize(create_objective(part2=False), 100)
    print(study_pt1.best_params, int(study_pt1.best_value))

    study_pt2 = optuna.create_study()
    study_pt2.optimize(create_objective(part2=True), 100)
    print(study_pt2.best_params, int(study_pt2.best_value))
