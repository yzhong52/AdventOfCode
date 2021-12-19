import PriorityQueue from "ts-priority-queue";
import { print_result, readGrid } from "./helpers";

let grid = readGrid(15)

let maxX = grid.length
let maxY = grid[0].length

class Pos {
    x: number
    y: number
    risk: number

    constructor(x: number, y: number, risk: number) {
        this.x = x
        this.y = y
        this.risk = risk
    }
}

let offsets = [
    [-1, 0],
    [1, 0],
    [0, 1],
    [0, -1]
]

function findLowestRisk(scale: number): number {
    let totalMaxX = maxX * scale
    let totalMaxY = maxY * scale

    function isValid(x: number, y: number) {
        return x >= 0 && x < totalMaxX && y >= 0 && y < totalMaxY
    }

    function riskAt(x: number, y: number): number {
        let increasedX = Math.floor(x / maxX)
        let increasedY = Math.floor(y / maxY)
        return (grid[x % maxX][y % maxY] + increasedX + increasedY - 1) % 9 + 1
    }

    let visited = new Array(totalMaxX)
    for (var i = 0; i < totalMaxX; i++) {
        visited[i] = new Array(totalMaxY).fill(false)
    }

    let visiting = new PriorityQueue<Pos>({ comparator: (p1, p2) => p1.risk - p2.risk });
    visiting.queue(new Pos(0, 0, 0))
    while (visiting.length > 0) {
        let pos = visiting.dequeue()
        if (visited[pos.x][pos.y]) {
            continue
        }
        if (pos.x == totalMaxX - 1 && pos.y == totalMaxY - 1) {
            return pos.risk
        }
        visited[pos.x][pos.y] = true
        for (let [dx, dy] of offsets) {
            let x = pos.x + dx
            let y = pos.y + dy

            if (isValid(x, y) && !visited[x][y]) {
                visiting.queue(new Pos(x, y, pos.risk + riskAt(x, y)))
            }
        }
    }
    return 0
}

print_result(15, 1, findLowestRisk(1))
print_result(15, 1, findLowestRisk(5))
