import { print_result, readStrings } from "./helpers";

let lines = readStrings(8);

let map: Map<number, string> = new Map();
map.set(0, "abcefg")
map.set(1, "cf");
map.set(2, "acdeg");
map.set(3, "acdfg")
map.set(4, "bcdf")
map.set(5, "abdfg")
map.set(6, "abdefg")
map.set(7, "acf")
map.set(8, "abcdefg")
map.set(9, "abcdfg")

let displayToDigit: Map<string, number> = new Map();

let lengthIndex: Map<number, Array<number>> = new Map();
for (let [key, value] of map) {
    var current: Array<number> = lengthIndex.get(value.length) ?? new Array();
    current.push(key);
    lengthIndex.set(value.length, current);

    displayToDigit.set(value, key);
}

let uniquesLength: Set<number> = new Set();
for (let [length, numbers] of lengthIndex) {
    if (numbers.length == 1) {
        uniquesLength.add(length);
    }
}

var part1 = 0
for (let line of lines) {
    let [_, values] = line.split(' | ')
    part1 += values.split(' ').filter(value => uniquesLength.has(value.length)).length
}
print_result(8, 1, part1);

// For part 2, since there are only 7 segments, we only have
// 7! = 5040 combinations. Let's generate them all.
var combinations: Array<string> = new Array("");
for (let char of "abcdefg") {
    var nextCombinations: Array<string> = new Array();
    for (let currentPattern of combinations) {
        let currentLength = currentPattern.length;
        for (var index = 0; index <= currentLength; index++) {
            let nextPattern = currentPattern.slice(0, index) + char + currentPattern.slice(index)
            nextCombinations.push(nextPattern);
        }
    }
    combinations = nextCombinations;
}

function convert(value: string, from: string, to: string): string {
    let mapping: Map<string, string> = new Map();
    for (var i = 0; i < from.length; i += 1) {
        mapping.set(from[i], to[i]);
    }

    return Array.from(value).map(char => mapping.get(char)!).sort().join('');
}

// For each of the assignment, calculate how the 10 digit is represented
let assignmentIndex: Map<string, string> = new Map();
for (let assignment of combinations) {
    let displays = Array.from(map.values()).map(value => convert(value, "abcdefg", assignment))
    let key = displays.sort().join(' ');
    assignmentIndex.set(key, assignment);
}

var part2 = 0;
for (let line of lines) {
    let [displays, values] = line.split(' | ')
    let key = displays.split(' ').map(display => {
        return Array.from(display).sort().join('')
    }).sort().join(" ")

    let index = assignmentIndex.get(key)!;

    var currentNumber = 0;
    values.split(' ').map(value => {
        let convertedDisplay = convert(value, index, "abcdefg")
        currentNumber = currentNumber * 10 + displayToDigit.get(convertedDisplay)!
    })
    part2 += currentNumber;
}
print_result(8, 2, part2);
