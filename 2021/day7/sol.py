import numpy as np
import optuna


def read_crab_positions(filename: str) -> np.array:

    with open(filename) as file:
        data = [line.rstrip().split(",") for line in file.readlines()]
    return np.array([d for d in data[0]], dtype=np.int32)


def objective_pt1(trial: optuna.Trial) -> int:

    pos = trial.suggest_int("pos", crabs.min(), crabs.max())
    cost = np.sum(np.abs(crabs - pos))
    return cost


def objective_pt2(trial: optuna.Trial) -> int:

    pos = trial.suggest_int("pos", crabs.min(), crabs.max())
    distance = np.abs(crabs - pos)
    cost = 0

    for dist in distance:
        rng = np.arange(1, dist + 1)
        cost += np.sum(rng)

    return cost


if __name__ == "__main__":
    crabs = read_crab_positions("d7.txt")
    optuna.logging.disable_default_handler()

    study_pt1 = optuna.create_study()
    study_pt1.optimize(objective_pt1, 100)
    print(study_pt1.best_params, study_pt1.best_value)

    study_pt2 = optuna.create_study()
    study_pt2.optimize(objective_pt2, 100)
    print(study_pt2.best_params, study_pt2.best_value)
