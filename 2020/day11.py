from time import sleep
from typing import List

file = open("day11.txt", "r")
initial_seats = [list(row.strip()) for row in file.readlines()]

height = len(initial_seats)
width = len(initial_seats[0])

empty = 'L'
occupied = '#'
floor = '.'


def visualize(iteration: int, data: List[List[str]], debug):
    if not debug:
        return
    print(f"Iteration {iteration}:")
    output = ""
    for row in data:
        for cell in row:
            if cell == floor:
                output += ' '
            elif cell == occupied:
                output += '#'
            elif cell == empty:
                output += '~'
        output += '\n'
    print(output)
    sleep(0.3)


def count_occupied(seats_data: List[List[str]]):
    return sum([seat == occupied for row in seats_data for seat in row])


def part1(seats: List[List[str]], debug: bool):
    previous_seats = []
    counter = 0
    while previous_seats != seats:
        new_seats = [['.'] * width for _ in range(height)]
        for i in range(height):
            for j in range(width):
                new_seats[i][j] = seats[i][j]
                # Floor (.) never changes; seats don't move, and nobody sits on the floor.
                if seats[i][j] != floor:
                    occupied_neighbour = 0
                    for x in range(max(0, i - 1), min(height, i + 2)):
                        for y in range(max(0, j - 1), min(width, j + 2)):
                            if x != i or y != j:
                                occupied_neighbour += seats[x][y] == occupied

                    if seats[i][j] == empty and occupied_neighbour == 0:
                        # If a seat is empty (L) and there are no occupied seats adjacent to it, the seat becomes
                        # occupied.
                        new_seats[i][j] = occupied
                    if seats[i][j] == occupied and occupied_neighbour >= 4:
                        # If a seat is occupied (#) and four or more seats adjacent to it are also occupied, the seat
                        # becomes empty.
                        new_seats[i][j] = empty
        counter += 1
        visualize(counter, new_seats, debug)
        seats, previous_seats = new_seats, seats

    print(f"Part 1: {count_occupied(seats_data=seats)}")


def change_neighbour(seat: str, current: int) -> int:
    if seat == occupied:
        return 1
    elif seat == empty:
        return 0
    elif seat == floor:
        # keep the current value
        return current


def part2(seats: List[List[str]], debug: bool):
    previous_seats = []
    counter = 0
    while previous_seats != seats:
        new_seats = [['.'] * width for _ in range(height)]

        # pre-calculate the number of visible neighbours in 8-directions
        occupied_neighbours = [[0] * width for _ in range(height)]

        def calculate_occupied(y: int, x: int, dy: int, dx: int):
            current = 0
            while 0 <= y < height and 0 <= x < width:
                occupied_neighbours[y][x] += current
                current = change_neighbour(seats[y][x], current)
                y += dy
                x += dx

        for i in range(height):
            for dy in [-1, 0, 1]:
                # avoid duplicates at corners
                if i in {0, height - 1} and dy in {-1, 1}:
                    continue
                calculate_occupied(y=i, x=0, dy=dy, dx=1)
                calculate_occupied(y=i, x=width - 1, dy=dy, dx=-1)
        for j in range(width):
            for dx in [-1, 0, 1]:
                calculate_occupied(y=0, x=j, dy=1, dx=dx)
                calculate_occupied(y=height - 1, x=j, dy=-1, dx=dx)
        for i in range(height):
            for j in range(width):
                new_seats[i][j] = seats[i][j]
                # Floor (.) never changes; seats don't move, and nobody sits on the floor.
                if seats[i][j] != floor:
                    if seats[i][j] == empty and occupied_neighbours[i][j] == 0:
                        # If a seat is empty (L) and there are no occupied seats adjacent to it, the seat becomes
                        # occupied.
                        new_seats[i][j] = occupied
                    if seats[i][j] == occupied and occupied_neighbours[i][j] >= 5:
                        # If a seat is occupied (#) and five or more seats adjacent to it are also occupied, the seat
                        # becomes empty.
                        new_seats[i][j] = empty
        counter += 1
        visualize(counter, new_seats, debug)
        seats, previous_seats = new_seats, seats
    print(f"Part 2: {count_occupied(seats_data=seats)}")


part1(seats=initial_seats, debug=False)
part2(seats=initial_seats, debug=True)
