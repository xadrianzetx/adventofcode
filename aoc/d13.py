import numpy as np


def pick_bus(depart: int, schedule: str) -> int:
    """
    What is the ID of the earliest bus you can
    take to the airport multiplied by the number of
    minutes you'll need to wait for that bus?
    """

    running = [int(b) for b in schedule.split(',') if b != 'x']
    delays = [b - depart % b for b in running]
    return running[np.argmin(delays)] * np.min(delays)
