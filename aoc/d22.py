import numpy as np


def play_game(decka: list, deckb: list) -> int:
    """
    top is right
    """

    while len(decka) > 0 and len(deckb) > 0:
        carda = decka.pop()
        cardb = deckb.pop()

        if carda > cardb:
            decka.insert(0, carda)
            decka.insert(0, cardb)
        
        else:
            deckb.insert(0, cardb)
            deckb.insert(0, carda)
    
    wins = decka if len(decka) > 0 else deckb
    rng = np.arange(1, len(wins) + 1)
    result = np.sum(np.array(wins) * rng)

    return result
