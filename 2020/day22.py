import collections
from typing import List, Deque


def parse() -> (List[int], List[int]):
    file = open("day22.txt", "r")
    cards = file.read().splitlines()
    idx = 0
    while cards[idx] != '':
        idx += 1
    return list(map(int, cards[1: idx])), list(map(int, cards[idx + 2:]))


def play_card_game(player1: List[int], player2: List[int], should_recurse: bool):
    player1 = collections.deque(player1)
    player2 = collections.deque(player2)

    seen = set()
    while player1 and player2:
        # If there was a previous round in this game that had exactly the same cards in the same order in the same
        # players' decks, the game instantly ends in a win for player 1
        state = tuple(player1), tuple(player2)
        if state in seen:
            return player1, []
        else:
            seen.add(state)

        c1 = player1.popleft()
        c2 = player2.popleft()

        if should_recurse and (c1 <= len(player1) and c2 <= len(player2)):
            # Make a copy of the next cards in their deck (the quantity of cards copied is equal to the number on
            # the card they drew to trigger the sub-game).
            sub_c1, sub_c2 = play_card_game(list(player1)[0:c1], list(player2)[0:c2], should_recurse=True)
            player1_win = len(sub_c1) > 0
        else:
            player1_win = c1 > c2

        if player1_win:
            player1.extend([c1, c2])
        else:
            player2.extend([c2, c1])

    return player1, player2


def calculate_answer(player1: Deque[int], player2: Deque[int]) -> int:
    winner = player1 if player1 else player2
    return sum((len(winner) - i) * c for i, c in enumerate(winner))


part1 = calculate_answer(*play_card_game(*parse(), should_recurse=False))
print(part1)  # 32815

part2 = calculate_answer(*play_card_game(*parse(), should_recurse=True))
print(part2)  # 30695
