import collections
import math
import re
from typing import List, Tuple, Iterable

import numpy as np

Tile = Tuple[int, List[str]]
Edges = (str, str, str, str)


def parse() -> List[Tile]:
    file = open("day20.txt", "r")
    lines = file.read().splitlines()

    tiles = []

    idx = 0
    while idx < len(lines):
        idx2 = idx
        while idx2 < len(lines) and lines[idx2] != '':
            idx2 += 1
        tile_id = int(re.search(r'\d+', lines[idx]).group())
        tile = lines[idx + 1: idx2]
        tiles.append((tile_id, tile))
        idx = idx2 + 1

    return tiles


def get_edges(tile_data: List[str]) -> Edges:
    col1 = "".join(map(lambda x: x[0], tile_data))
    col2 = "".join(map(lambda x: x[-1], tile_data))
    return tile_data[0], tile_data[-1], col1, col2


def part1_corner_tiles(tiles: List[Tile]):
    # Count the number of times an edge appear in tiles
    edge_count = collections.defaultdict(int)
    for idx, tile in tiles:
        edges = get_edges(tile)
        for edge in edges:
            edge_count[tuple(edge)] += 1
            edge_count[tuple(edge[::-1])] += 1

    result = []
    for idx, tile in tiles:
        edges = get_edges(tile)
        shared_edges = 0
        for edge in edges:
            shared_edges += edge_count[tuple(edge)] > 1

        if shared_edges <= 2:
            print(f"{idx} is a corner tile.")
            result.append(idx)
    return result


def transforms():
    return [lambda x: x] + [np.rot90] * 3 + [np.fliplr] + [np.rot90] * 3


def stitch_tiles(tiles: List[Tile]):
    # Get list of indices for all the tiles that shared the edge
    shared_edge = collections.defaultdict(list)

    # Create a list of numpy arrays (for flipping and rotation)
    np_tile: List[np.ndarray] = []

    for idx, (_, tile) in enumerate(tiles):
        edges = get_edges(tile)
        for edge in edges:
            shared_edge[tuple(edge)].append(idx)
            shared_edge[tuple(edge[::-1])].append(idx)
        np_tile.append(np.array([list(row) for row in tile]))

    # Start with tile 0 at the origin, and not rotated
    visited = {(0, 0): 0}

    def dfs(x: int, y: int):
        tid = visited[(x, y)]
        current_tile = np_tile[tid]

        offsets = [(-1, 0), (1, 0), (0, -1), (0, 1)]

        left = lambda g: tuple(g[:, 0])
        right = lambda g: tuple(g[:, -1])
        top = lambda g: tuple(g[0, :])
        bottom = lambda g: tuple(g[-1, :])
        match_edges = [(top, bottom), (bottom, top), (left, right), (right, left)]

        for (dx, dy), (from_edge, to_edge) in zip(offsets, match_edges):
            current_edge = from_edge(current_tile)
            if (x + dx, y + dy) not in visited and len(shared_edge[current_edge]) == 2:
                shared_edge[current_edge].remove(tid)
                neighbour_id = shared_edge[current_edge].pop()

                visited[(x + dx, y + dy)] = neighbour_id
                for transform in transforms():
                    np_tile[neighbour_id] = transform(np_tile[neighbour_id])
                    if current_edge == to_edge(np_tile[neighbour_id]):
                        dfs(x + dx, y + dy)
                        break

    dfs(x=0, y=0)

    min_x = min(map(lambda pos: pos[0], visited.keys()))
    max_x = max(map(lambda pos: pos[0], visited.keys()))
    min_y = min(map(lambda pos: pos[1], visited.keys()))
    max_y = max(map(lambda pos: pos[1], visited.keys()))

    rows = []
    for x in range(min_x, max_x + 1):
        row_tiles = [np_tile[visited[(x, y)]][1:-1, 1:-1] for y in range(min_y, max_y + 1)]
        row = np.concatenate(row_tiles, axis=1)
        rows.append(row)
    return np.concatenate(rows)


def part2_detect_monster(big_tile: np.array, monster: List[str]):
    monster_pos = [(i, j) for i, row in enumerate(monster) for j, c in enumerate(row) if c == '#']
    monster_height = len(monster)
    monster_width = len(monster[0])

    height, width = big_tile.shape
    for transform in transforms():
        big_tile = transform(big_tile)
        for x in range(height - monster_height):
            for y in range(width - monster_width):
                if all([big_tile[x + i][y + j] != '.' for i, j in monster_pos]):
                    for i, j in monster_pos:
                        big_tile[x + i][y + j] = 'O'

    return big_tile


parsed_tiles = parse()
corner_tiles = part1_corner_tiles(parsed_tiles)
part1 = math.prod(corner_tiles)
print(part1)  # 66020135789767

stitched = stitch_tiles(parsed_tiles)

monster_patten = [
    "                  # ",
    "#    ##    ##    ###",
    " #  #  #  #  #  #   "
]

with_monsters = part2_detect_monster(stitched, monster_patten)
part2 = sum([with_monsters[x][y] == '#' for x in range(with_monsters.shape[0]) for y in range(with_monsters.shape[1])])
for row in np.flipud(np.rot90(with_monsters)):
    for c in row:
        print(c if c == 'O' else ' ', end='')
    print()
print(part2)
