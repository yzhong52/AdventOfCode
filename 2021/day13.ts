import { print_result, readStrings } from "./helpers";

let lines = readStrings(13)
let emptyLineIndex = lines.indexOf("")

let numbers: Array<[number, number]> = lines.slice(0, emptyLineIndex).map(line => line.split(",").map(value => parseInt(value)) as [number, number])
console.log(numbers)

enum Direction { x, y }
class Folding {
    direction: Direction
    value: number

    constructor(match: RegExpMatchArray) {
        switch (match.groups?.direction) {
            case 'x':
                this.direction = Direction.x
                break;
            case 'y':
                this.direction = Direction.y
                break;
            default:
                throw new Error(`Invalid direction value {match.groups?.direction}`);
        }

        this.value = parseInt(match.groups!.value)
    }
}

let regex = /fold along (?<direction>x|y)=(?<value>[0-9]+)/
let folds = lines.slice(emptyLineIndex + 1).map(line => new Folding(line.match(regex)!))

function fold1(folding: Folding, point: [number, number]): [number, number] {
    switch (folding.direction) {
        case Direction.x:
            if (folding.value < point[0]) {
                return [2 * folding.value - point[0], point[1]]
            } else {
                return point
            }
        case Direction.y:
            if (folding.value < point[1]) {
                return [point[0], 2 * folding.value - point[1]]
            }
            else {
                return point
            }
    }
}

function folding(folding: Folding, points: Array<[number, number]>): Array<[number, number]> {
    let map = new Map<string, [number, number]>();
    for (let point of points) {
        let foldedPoint = fold1(folding, point as [number, number])
        map.set(foldedPoint.toString(), foldedPoint)
    }
    return Array.from(map.values());
}

let part1 = folding(folds[0], numbers)
print_result(13, 1, part1.length);

let part2: Array<[number, number]> = folds.reduce((previous, current) => folding(current, previous), numbers)
let maxX = Math.max(...part2.map(point => point[0])) + 1
let maxY = Math.max(...part2.map(point => point[1])) + 1
let image = new Array<Array<string>>(maxY);
for (var j = 0; j < maxY; j += 1) {
    image[j] = new Array(maxX).fill(' ')
}

for (let point of part2) {
    image[point[1]][point[0]] = 'X'
}

print_result(13, 2, image.map(row => row.join('')).join('\n'));
