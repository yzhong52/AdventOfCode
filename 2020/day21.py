import collections
from functools import reduce
from typing import List, Tuple


def parse():
    file = open("day21.txt", "r")
    foods = file.read().splitlines()

    import re
    for food in foods:
        ingredients, allergens = re.search(r'(.+?) \(contains (.+?)\)', food).groups()
        yield ingredients.split(' '), allergens.split(', ')


def find_dangerous_ingredients(foods: List[Tuple[List[str], List[str]]]):
    ingredients_counter = reduce(
        lambda x, y: x + y,
        map(lambda x: collections.Counter(x[0]), foods),
        collections.Counter()
    )

    match = {}
    for ingredients, allergens in foods:
        for allergen in allergens:
            if allergen not in match:
                match[allergen] = set(ingredients)
            else:
                match[allergen] = match[allergen].intersection(set(ingredients))

    potential_ingredients = reduce(lambda x, y: x.union(y), match.values(), set())
    harmful_ingredients = set(ingredients_counter.keys()) - potential_ingredients

    # The number of times any of these ingredients appear in any ingredients list
    result_part1 = sum([ingredients_counter[i] for i in harmful_ingredients])

    dangerous_matches = []
    while match:
        matched_allergen = next(allergen for allergen, ingredients in match.items() if len(ingredients) == 1)
        (matched_ingredient,) = match[matched_allergen]  # there is only one

        dangerous_matches.append((matched_allergen, matched_ingredient))

        for allergen in list(match.keys()):
            match[allergen].discard(matched_ingredient)
            if len(match[allergen]) == 0:
                del match[allergen]

    # The canonical dangerous ingredient list
    result_part2 = ",".join(map(lambda x: x[1], sorted(dangerous_matches)))

    return result_part1, result_part2


part1, part2 = find_dangerous_ingredients(list(parse()))
print(part1, part2)
