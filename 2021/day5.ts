import { print_result, readStrings } from "./helpers";

let data = readStrings(5)


class Point {
    x: number;
    y: number;

    static parse(data: string): Point {
        let [x, y] = data.split(',').map(value => parseInt(value))
        return new Point(x, y);
    }

    constructor(x: number, y: number) {
        this.x = x;
        this.y = y;
    }
}

class Line {
    p1: Point;
    p2: Point;

    constructor(data: string) {
        let [p1, p2] = data.split(' -> ').map(value => Point.parse(value));
        this.p1 = p1;
        this.p2 = p2;
    }

    get delta() {
        let delta_x = this.p2.x - this.p1.x;
        let delta_y = this.p2.y - this.p1.y;
        let length = Math.sqrt(delta_x * delta_x + delta_y * delta_y);
        return [delta_x / length, delta_y / length];
    }
}

let inputLines = data.map(lineData => new Line(lineData))

let horizontal_vertical_lines = inputLines
    .filter(line => line.p1.x == line.p2.x || line.p1.y == line.p2.y)

let xs = inputLines.map(line => Math.max(line.p1.x, line.p2.x));
let ys = inputLines.map(line => Math.max(line.p1.y, line.p2.y));
let maxX = Math.max(...xs);
let maxY = Math.max(...ys);
let draw: Array<Array<number>> = new Array(maxX + 1);
for (let rowId = 0; rowId <= maxX; rowId += 1){
    draw[rowId] = new Array(maxY + 1).fill(0);
}

let map: Map<number, number> = new Map();
for (let line of horizontal_vertical_lines) {
    let [delta_x, delta_y] = line.delta;
    for (let x = line.p1.x, y = line.p1.y; x != line.p2.x + delta_x || y != line.p2.y + delta_y; x += delta_x, y += delta_y) {
        let hash = x * (maxY + 1) + y;
        let count = map.get(hash) ?? 0
        draw[x][y] = draw[x][y] + 1;
        map.set(hash, count + 1)
    }
}

let part1 = Array.from(map.values()).reduce((sum, cur) => (cur >= 2) ? sum + 1 : sum, 0);
print_result(5, 1, part1);
