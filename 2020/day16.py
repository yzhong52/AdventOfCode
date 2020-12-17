import collections
from typing import List

file = open("day16.txt", "r")
lines = file.readlines()
rules = collections.defaultdict(list)

idx = 0
while lines[idx] != '\n':
    line = lines[idx]
    idx += 1
    # e.g. "class: 1 - 3 or 5 - 7"
    rule, numbers = line.split(": ")
    for pair in numbers.split(" or "):
        left, right = pair.split("-")
        r = int(left), int(right)
        rules[rule].append(r)


def parse_digits(line: str) -> List[int]:
    return [int(num) for num in line.split(',')]


idx += 2  # get rid of empty line and title line
your_ticket = parse_digits(lines[idx])

idx += 3  # get rid of the previous line, an empty line and title line
nearby_tickets = [parse_digits(line) for line in lines[idx:]]

all_ranges = sum(rules.values(), [])

part1 = 0  # result for part 1
valid_tickets = []  # collect the valid ticket as well, which is used in part 2

for ticket in nearby_tickets:
    is_valid = True
    for field in ticket:
        if not any(left <= field <= right for left, right in all_ranges):
            is_valid = False
            part1 += field

    if is_valid:
        valid_tickets.append(ticket)

print(part1)

# In part 2, we only care about the `departure` fields
num_fields = len(your_ticket)

# flags[i][j] whether slot i can be assign to field j
flags = [set(rules.keys()) for _ in range(num_fields)]

for rule, ranges in rules.items():
    for ticket in valid_tickets:
        for i, field in enumerate(ticket):
            if not any(left <= field <= right for left, right in ranges):
                flags[i].remove(rule)

assignment = [''] * num_fields
for i in range(num_fields):
    # find the one with single element
    idx = 0
    while len(flags[idx]) != 1:
        idx += 1
    known_field = flags[idx].pop()
    for j in range(num_fields):
        flags[j].discard(known_field)
    assignment[idx] = known_field

part2 = 1
for i, field in enumerate(assignment):
    if field.startswith('departure'):
        part2 *= your_ticket[i]
print(part2)
