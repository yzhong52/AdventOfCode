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

    add(point: Point) {
        this.x += point.x;
        this.y += point.y;
    }

    equals(point: Point) {
        return this.x == point.x && this.y == point.y;
    }

    get hash() {
        // It is unfortunate that we don't have hash interface in typescript
        return `${this.x},${this.y}`
    }

    clone() {
        // It is unfortunate that we don't have an easy way to clone object in typescript
        // https://stackoverflow.com/questions/28150967/typescript-cloning-object
        return new Point(this.x, this.y);
    }
}

class Line {
    p1: Point;
    p2: Point;

    static parse(data: string): Line {
        let [p1, p2] = data.split(' -> ').map(value => Point.parse(value));
        return new Line(p1, p2);
    }

    constructor(p1: Point, p2: Point) {
        this.p1 = p1;
        this.p2 = p2;
    }

    delta(): Point {
        function normalize(value: number) {
            return value == 0 ? value : value / Math.abs(value);
        }

        return new Point(
            normalize(this.p2.x - this.p1.x),
            normalize(this.p2.y - this.p1.y)
        )
    }
}

let inputLines = data.map(Line.parse)

function add_to_map(map: Map<string, number>, p: Point) {
    let hash = `${p.x}@${p.y}`;
    let count = map.get(hash) ?? 0
    map.set(hash, count + 1);
}

function counter_intersections(lines: Array<Line>) {
    let map: Map<string, number> = new Map();
    for (let line of lines) {
        let delta = line.delta();
        for (let p = line.p1.clone(); !p.equals(line.p2); p.add(delta)) {
            add_to_map(map, p);
        }
        add_to_map(map, line.p2);
    }

    return Array.from(map.values()).reduce((sum, cur) => (cur >= 2) ? sum + 1 : sum, 0);
}

let horizontal_vertical_lines = inputLines
    .filter(line => line.p1.x == line.p2.x || line.p1.y == line.p2.y);
let part1 = counter_intersections(horizontal_vertical_lines);
print_result(5, 1, part1);

let horizontal_vertical_diagonal_lines = inputLines
    .filter(line => line.p1.x == line.p2.x || line.p1.y == line.p2.y || Math.abs(line.p1.y - line.p2.y) == Math.abs(line.p1.x - line.p2.x));
let part2 = counter_intersections(horizontal_vertical_diagonal_lines);
print_result(5, 2, part2);
