import numpy as np
from scipy.ndimage import generic_filter


def parse_input(data: list) -> np.array:
    """Reads cube map"""

    maps = np.array([list(d) for d in data])
    maps[maps == '.'] = 0
    maps[maps == '#'] = 1

    return maps.astype('int')


def switch(arr: np.array) -> int:
    """Applies rules to a cube"""

    if arr[len(arr) // 2] == 1:
        return 0 if arr.sum() - 1 not in [2, 3] else 1

    else:
        return 1 if arr.sum() == 3 else 0


def boot_process(init_map: np.array, dim: int) -> int:
    """
    Starting with your given initial configuration,
    simulate six cycles in a n-dimensional space.
    How many cubes are left in the active state after the sixth cycle?
    """

    # space is finite but big enough
    space_size = 40
    pad = init_map.shape[0]
    kenrel = np.ones([3] * dim, dtype=np.uint8)
    space = np.zeros(shape=(space_size, ) * dim, dtype=np.uint8)
    idx = np.array([space_size] * dim) // 2
    row, col, *depth = idx

    if dim == 3:
        space[row:row + pad, col:col + pad, depth[0]] = init_map

    else:
        space[row:row + pad, col:col + pad, depth[0], depth[1]] = init_map

    for _ in range(6):
        # convolve space with kernel, applying
        # switch function at each step
        space = generic_filter(space, switch, footprint=kenrel,
                               mode='constant', cval=0)

    return space.sum()
