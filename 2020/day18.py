from typing import List, Union, Set


def calculate(operator: str, value1: int, value2: int) -> int:
    if operator == '*':
        return value1 * value2
    elif operator == '+':
        return value1 + value2
    raise ArithmeticError(f"Not valid operator {operator}")


def compact(result: List[Union[str, int]], operators: Set[str]):
    while len(result) >= 3 and result[-2] in operators and type(result[-1]) is int:
        value2 = result.pop()
        operator = result.pop()
        value1 = result.pop()
        result.append(calculate(operator=operator, value1=value1, value2=value2))


def evaluate(line: str, high_precedence_operators: Set[str]) -> int:
    result: List[Union[str, int]] = []

    for c in line:
        if c == ' ':
            continue
        if c.isdigit():
            result.append(int(c))
        elif c == ')':
            compact(result, {'+', '*'})
            # remove the '('
            result[-2] = result[-1]
            result.pop()
        else:
            result.append(c)

        compact(result, high_precedence_operators)

    compact(result, {'+', '*'})
    return result[-1]


def evaluate_part1(line: str) -> int:
    return evaluate(line, {'+', '*'})


def evaluate_part2(line: str) -> int:
    return evaluate(line, {'+'})


print(evaluate_part1("1 + 2 * 3 + 4 * 5 + 6"), 71)
print(evaluate_part1("1 + (2 * 3) + (4 * (5 + 6))"), 51)
print(evaluate_part1("2 * 3 + (4 * 5)"), 26)
print(evaluate_part1("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 437)
print(evaluate_part1("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"), 13632)

print(evaluate_part2("1 + 2 * 3 + 4 * 5 + 6"), 231)
print(evaluate_part2("1 + (2 * 3) + (4 * (5 + 6))"), 51)
print(evaluate_part2("2 * 3 + (4 * 5)"), 46)
print(evaluate_part2("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 1445)
print(evaluate_part2("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"), 669060)
print(evaluate_part2("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"), 23340)

file = open("day18.txt", "r")
lines = file.readlines()

part1 = sum(map(lambda line: evaluate_part1(line.strip()), lines))
print(part1)  # 6923486965641

part2 = sum(map(lambda line: evaluate_part2(line.strip()), lines))
print(part2)  # 70722650566361
