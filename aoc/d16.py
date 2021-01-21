import re
import numpy as np


def parse_rules(rules: list) -> list:
    """Parses ticket rules and ranges"""

    valid_vals = np.array([], dtype='int')

    for rule in rules:
        # rle = re.findall('.*:', rule)[0][:-1]
        rng = re.findall(': (.*)', rule)[0].split(' or ')

        for r in rng:
            low, high = r.split('-')
            rngarr = np.arange(int(low), int(high) + 1)
            valid_vals = np.append(rngarr, valid_vals)

    return valid_vals


def parse_tickets(tickets: list) -> list:
    """Parses ticket contents"""

    all_fields = []

    for ticket in tickets:
        fields = np.array(ticket.split(','), dtype='int')
        all_fields.append(fields)

    return np.array(all_fields)


def find_invalid_rate(tickets: np.array, valid_fields: list) -> int:
    """
    Consider the validity of the nearby tickets you scanned.
    What is your ticket scanning error rate?
    """

    rate = 0

    for ticket in tickets:
        valid = np.array([f in valid_fields for f in ticket])
        passed = all(valid)

        if not passed:
            r = np.sum(ticket[~valid])
            rate += r

    return rate
