import collections
from functools import lru_cache

file = open("day7.txt", "r")
rules = file.readlines()

outer_bag_map = collections.defaultdict(list)

for rule in rules:
    outer_bag, inner_bags = rule.split(" contain ")
    # e.g. outer_bug = "clear tan bags", drop the last word
    outer_bag = " ".join(outer_bag.split(" ")[:-1])
    for inner_bag in inner_bags.split(", "):
        # e.g. inner_bag = "5 mirrored maroon bags", drop the first and last word here.
        # e.g. inner_bag = "no other bags", we can skip this
        if inner_bag == "no other bags.\n":
            continue
        inner_bag = " ".join(inner_bag.split(" ")[1:-1])
        outer_bag_map[inner_bag].append(outer_bag)

part1 = set()
stack = ["shiny gold"]
while stack:
    current = stack.pop()
    for outer_bag in outer_bag_map[current]:
        if outer_bag not in part1:
            part1.add(outer_bag)
            stack.append(outer_bag)
print(len(part1))

inner_bags_map = collections.defaultdict(list)
for rule in rules:
    outer_bag, inner_bags = rule.split(" contain ")
    # e.g. outer_bug = "clear tan bags", drop the last word
    outer_bag = " ".join(outer_bag.split(" ")[:-1])

    assert outer_bag not in inner_bags_map, (outer_bag, inner_bags_map)
    for inner_bag_desc in inner_bags.split(", "):
        # e.g. inner_bag = "5 mirrored maroon bags", drop the first and last word here.
        # e.g. inner_bag = "no other bags", we can skip this
        if inner_bag_desc == "no other bags.\n":
            continue
        words = inner_bag_desc.split(" ")
        inner_bag_count = int(words[0])
        inner_bag = " ".join(words[1:-1])
        inner_bags_map[outer_bag].append((inner_bag_count, inner_bag))


@lru_cache(None)
def total_contained_bags(bag: str) -> int:
    total = 0
    for bag_count, inner_bag in inner_bags_map[bag]:
        total += bag_count + bag_count * total_contained_bags(inner_bag)
    return total


part2 = total_contained_bags("shiny gold")
print(part2)
