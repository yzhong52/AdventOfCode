from typing import List


class Cup:
    def __init__(self, value: int):
        self.pre = None
        self.nxt = None
        self.value = value


def decrease_one(cup_value: int, max_value: int) -> int:
    return cup_value - 1 if cup_value != 1 else max_value


def take(cup: Cup, num: int) -> List[int]:
    joined = []
    for _ in range(num):
        joined.append(cup.value)
        cup = cup.nxt
    return joined


def simulate(cup_values: List[int], cup_count: int, moves: int, output_size: int) -> List[int]:
    first_cup = Cup(cup_values[0])
    last_cup = first_cup
    cups_map = {first_cup.value: first_cup}
    for i in range(1, cup_count):
        if i < len(cup_values):
            new_cup = Cup(cup_values[i])
        else:
            new_cup = Cup(i + 1)
        last_cup.nxt = new_cup
        new_cup.pre = last_cup
        last_cup = new_cup
        cups_map[new_cup.value] = new_cup
    first_cup.pre = last_cup
    last_cup.nxt = first_cup

    current_cup = first_cup

    for rnd in range(moves):
        head = current_cup.nxt
        tail = current_cup.nxt.nxt.nxt

        # Picks up the three cups that are immediately clockwise of the current cup
        tail.nxt.prv = current_cup
        current_cup.nxt = tail.nxt
        pick_up = [head.value, head.nxt.value, head.nxt.nxt.value]

        # Selects a destination cup
        destination_cup_value = decrease_one(current_cup.value, cup_count)
        while destination_cup_value in pick_up:
            destination_cup_value = decrease_one(destination_cup_value, cup_count)

        # Places the cups it just picked up so that they are immediately clockwise of the destination cup
        destination_cup = cups_map[destination_cup_value]
        tail.nxt = destination_cup.nxt
        destination_cup.nxt.prv = tail
        head.prv = destination_cup
        destination_cup.nxt = head

        # Selects a new current
        current_cup = current_cup.nxt

    return take(cups_map[1].nxt, output_size)


part1 = simulate(list(map(int, "916438275")), cup_count=10, moves=100, output_size=10)
print("".join(map(str, part1)))  # 39564287

first, second = simulate(list(map(int, "916438275")), cup_count=1000000, moves=10000000, output_size=2)
part2 = first * second
print(part2)  # 404431096944
