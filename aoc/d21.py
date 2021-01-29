import re
import numpy as np


def build_ingredients_list(foods: list) -> dict:
    """
    Builds ingredient to allergen and
    allergen to ingredient maps
    """

    ing_dict = {}
    alg_dict = {}

    for food in foods:
        allergens = re.findall(r'\((.*?)\)', food.replace(',', ''))
        allergens = allergens[0].split()[1:]
        ingredients = re.findall(r'.*? \(', food)
        ingredients = ingredients[0].replace(' (', '').split()

        for ingredient in ingredients:
            if ingredient not in ing_dict.keys():
                ing_dict[ingredient] = 0

            ing_dict[ingredient] += 1

        for allergen in allergens:
            if allergen not in alg_dict.keys():
                alg_dict[allergen] = {'count': 0, 'ing': []}

            for ingredient in ingredients:
                alg_dict[allergen]['ing'].append(ingredient)

            alg_dict[allergen]['count'] += 1

    return alg_dict, ing_dict


def get_safe_foods(allergens: dict, ingredients: dict) -> tuple:
    """
    Determine which ingredients cannot
    possibly contain any of the allergens in your list.
    """

    unsafe = []

    for ingredient, attr in allergens.items():
        ing, counts = np.unique(attr['ing'], return_counts=True)
        ing = ing[counts == attr['count']]

        for i in ing:
            unsafe.append(i)

    safe = {k: v for k, v in ingredients.items() if k not in unsafe}
    count = sum(list(safe.values()))

    return count, safe


def get_unsafe_foods(allergens: dict, safe: list) -> str:
    """
    Find canonical list of unsafe ingredients
    """

    unsafe = {}
    candidates = {}

    for alg, attr in allergens.items():
        # filter ingredients list from confirmed safe
        x = [i for i in attr['ing'] if i not in safe.keys()]
        ing, counts = np.unique(x, return_counts=True)
        ing = ing[counts == attr['count']]
        if alg not in candidates.keys():
            candidates[alg] = []
        candidates[alg] = ing

    while True:
        # get one to one
        cands = [k for k, v in candidates.items() if len(v) == 1]

        if len(cands) == 0:
            # finished
            break

        for cand in cands:
            # match confirmed par
            ing = candidates[cand][0]
            unsafe[ing] = cand
            for alg in candidates.keys():
                # pop matched
                candidates[alg] = [a for a in candidates[alg] if a != ing]

    sitems = sorted(unsafe.items(), key=lambda item: item[1])
    sitems = [item[0] for item in sitems]
    canon = ','.join(sitems)

    return canon
