import collections
from typing import List, Iterator, Dict, Tuple

directions = {"e": (2, 0), "se": (1, -1), "sw": (-1, -1), "w": (-2, 0), "nw": (-1, 1), "ne": (1, 1)}


def parse() -> Iterator[List[str]]:
    file = open("day24.txt")
    lines = file.read().splitlines()
    for line in lines:
        instruction = []
        i = 0
        while i < len(line):
            if i + 1 < len(line) and line[i: i + 2] in directions:
                instruction.append(line[i: i + 2])
                i += 2
            else:
                instruction.append(line[i])
                i += 1
        yield instruction


def navigate(steps: List[str]) -> (int, int):
    x, y = 0, 0
    for step in steps:
        dx, dy = directions[step]
        x += dx
        y += dy
    return x, y


def part1_flip_tiles() -> Dict[Tuple[int, int], bool]:
    tiles = collections.defaultdict(bool)
    for instruction in parse():
        pos = navigate(instruction)
        tiles[pos] = not tiles[pos]
    return tiles


def daily_flip_tile_rule(x: int, y: int, state: Dict[Tuple[int, int], bool]) -> bool:
    black_tiles = sum(state[(x + dx, y + dy)] for dx, dy in directions.values())
    if state[(x, y)] and (black_tiles == 0 or black_tiles > 2):
        return False
    elif not state[(x, y)] and black_tiles == 2:
        return True
    else:
        return state[(x, y)]


def part2_mutate(state: Dict[Tuple[int, int], bool], days: int):
    for day in range(days):
        new_state: Dict[(int, int), bool] = collections.defaultdict(bool)
        for x, y in list(state.keys()):
            for x1, y1 in [(x, y)] + [(x + dx, y + dy) for dx, dy in directions.values()]:
                if (x1, y1) not in new_state:
                    new_state[(x1, y1)] = daily_flip_tile_rule(x1, y1, state)
        state = new_state
        print(f"Day {day + 1}: {sum(state.values())}")
    return state


day1 = part1_flip_tiles()
part1 = sum(day1.values())
print(part1)  # 455

day100 = part2_mutate(day1, days=100)
part2 = sum(day100.values())
print(part2)  # 3904
