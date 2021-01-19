import re
import numpy as np


def emulate_decoder(program: list) -> int:
    """
    Execute the initialization program.
    What is the sum of all values left
    in memory after it completes?
    (Do not truncate the sum to 36 bits.)
    """

    mask = None
    mem = np.zeros((128000, )).astype('int64')

    for op in program:
        if op[:3] == 'mem':
            val = int(re.findall('= (.*)', op)[0])
            addr = int(re.findall('\[(.*?)\]', op)[0])

            for pos, bit in enumerate(mask[::-1]):
                if bit == 'X':
                    continue
                elif int(bit) == 0:
                    # clear bit at pos
                    val &= ~(1 << pos)
                else:
                    # set bit at pos
                    val |= (1 << pos)

            mem[addr] = val

        else:
            mask = re.findall('= (.*)', op)[0]

    return sum(mem)
