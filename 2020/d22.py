import numpy as np


def play_game(decka: list, deckb: list) -> int:
    """Plays vanilla combat game"""

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


def play_round(decka: list, deckb: list) -> tuple:
    """Plays combat round with recursive rules"""

    # previous round memory
    mem = []

    while len(decka) > 0 and len(deckb) > 0:
        if decka in mem and deckb in mem:
            # player 1 wins!
            return True, decka

        else:
            # save state
            mem.append(decka.copy())
            mem.append(deckb.copy())

        carda = decka.pop()
        cardb = deckb.pop()

        if len(decka) >= carda and len(deckb) >= cardb:
            playera_wins, _ = play_round(decka[-carda:], deckb[-cardb:])

        else:
            playera_wins = carda > cardb

        if playera_wins:
            decka.insert(0, carda)
            decka.insert(0, cardb)

        else:
            deckb.insert(0, cardb)
            deckb.insert(0, carda)

    playera_wins = len(decka) > 0
    winning_deck = decka if len(decka) != 0 else deckb

    return playera_wins, winning_deck


def play_recursive_game(decka: list, deckb: list) -> int:
    """
    Defend your honor as Raft Captain by playing
    the small crab in a game of Recursive Combat
    """

    _, deck = play_round(decka, deckb)
    rng = np.arange(1, len(deck) + 1)
    result = np.sum(np.array(deck) * rng)

    return result
