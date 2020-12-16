import collections
from enum import Enum


class Operation(Enum):
    mask = 1
    mem = 2


def parse():
    file = open("day14.txt", "r")
    lines = file.readlines()

    for line in lines:
        operation, value = line.split(" = ")
        if operation == "mask":
            yield Operation.mask, value.strip()
        else:
            value = int(value)
            address = int(operation[4:-1])  # i.e. mem[1157] => 1157
            yield Operation.mem, address, value


def part1():
    memory_part1 = collections.defaultdict(int)
    mask_and, mask_or = 0, 0

    for operation, *numbers in parse():
        if operation == Operation.mask:
            mask_and, mask_or = 0, 0
            mask = numbers[0]
            for bit in mask:
                mask_and = (mask_and << 1) + (bit != '0')
                mask_or = (mask_or << 1) + (bit == '1')
        else:
            address, value = numbers
            memory_part1[address] = value & mask_and | mask_or

    return sum(memory_part1.values())


def part2():
    memory_part2 = collections.defaultdict(int)

    mask = ""
    for operation, *numbers in parse():
        if operation == Operation.mask:
            mask = numbers[0]
        else:
            address, value = numbers
            addresses = [0]
            pos_mask = 1 << len(mask)
            for mi, m in enumerate(mask):
                pos_mask >>= 1
                if m == 'X':
                    tmp = []
                    for cur in addresses:
                        tmp.append(cur * 2)
                        tmp.append(cur * 2 + 1)
                    addresses = tmp
                elif m == '1':
                    for i in range(len(addresses)):
                        addresses[i] = addresses[i] * 2 + 1
                elif m == '0':
                    last_bit = address & pos_mask
                    for i in range(len(addresses)):
                        addresses[i] = addresses[i] * 2 + (last_bit != 0)

            for address in addresses:
                memory_part2[address] = value

    return sum(memory_part2.values())


print(part1())
print(part2())
