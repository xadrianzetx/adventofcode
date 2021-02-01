import math
from typing import Optional


def encrypt(x: int, g: int) -> int:
    """
    http://pi.math.cornell.edu/~mec/2003-2004/cryptography/diffiehellman/diffiehellman.html
    """
    return (g ** x) % 20201227


def bsgs(g: int, h: int, p: int) -> Optional[int]:
    """
    Solve for x in h = g^x mod p
    https://en.wikipedia.org/wiki/Baby-step_giant-step    
    """

    # baby step
    m = math.ceil(math.sqrt(p - 1))
    lookup = {pow(g, i, p): i for i in range(m)}

    # precompute via Fermat's Little Theorem
    c = pow(g, m * (p - 2), p)

    # giant step
    for i in range(m):
        y = (h * pow(c, i, p)) % p
        if y in lookup:
            return i * m + lookup[y]

    return None
