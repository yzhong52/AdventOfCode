from dataclasses import dataclass
from typing import List

file = open("day8.txt", "r")
lines = file.readlines()


@dataclass
class Instruction:
    operation: str
    value: int

    def swap_jmp_nop(self):
        self.operation = 'jmp' if self.operation != 'jmp' else 'nop'


parsed_instructions: List[Instruction] = []
for line in lines:
    instruction, value = line.split(' ')
    parsed_instructions.append(Instruction(instruction, int(value)))


def compute_accumulator(instructions: List[Instruction]):
    visited = set()
    accumulator = 0
    i = 0
    while i not in visited and i < len(instructions):
        visited.add(i)
        operation = instructions[i].operation
        if operation == "nop":
            i += 1
        elif operation == 'jmp':
            i += instructions[i].value
        elif operation == 'acc':
            accumulator += instructions[i].value
            i += 1
        else:
            assert False
    return accumulator


# O(N)
def fix_instructions(instructions: List[Instruction]) -> List[Instruction]:
    # Find all points on the loop
    loop = set()
    cur = 0
    while cur not in loop:
        loop.add(cur)
        if instructions[cur].operation == 'jmp':
            cur += instructions[cur].value
        else:
            cur += 1

    visited = loop.copy()
    for i in loop:
        if instructions[i].operation == 'acc':
            continue

        # Otherwise, let's see if we can exist the loop
        cur = i
        instructions[i].swap_jmp_nop()
        visited.remove(i)

        while cur not in visited and cur < len(instructions):
            visited.add(cur)
            if instructions[cur].operation == 'jmp':
                cur += instructions[cur].value
            else:
                cur += 1

        if cur == len(instructions):
            print(f"Found the buggy instruction at {i}")
            return instructions
        else:
            instructions[i].swap_jmp_nop()

    assert False, "Count not find the buggy instruction"


part1 = compute_accumulator(parsed_instructions)
print(part1)

fixed_instructions = fix_instructions(parsed_instructions)
part2 = compute_accumulator(fixed_instructions)
print(part2)
