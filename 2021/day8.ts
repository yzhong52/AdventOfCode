import { print_result, readStrings } from "./helpers";

let lines = readStrings(8);

const DEFAULT_ASSIGNMENT = "abcdefg";

let digitToDisplay: Map<number, string> = new Map();
digitToDisplay.set(0, "abcefg")
digitToDisplay.set(1, "cf");
digitToDisplay.set(2, "acdeg");
digitToDisplay.set(3, "acdfg")
digitToDisplay.set(4, "bcdf")
digitToDisplay.set(5, "abdfg")
digitToDisplay.set(6, "abdefg")
digitToDisplay.set(7, "acf")
digitToDisplay.set(8, "abcdefg")
digitToDisplay.set(9, "abcdfg")

let displayToDigit: Map<string, number> = new Map();

let lengthIndex: Map<number, Array<number>> = new Map();
for (let [key, value] of digitToDisplay) {
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
var assignmentCombinations: Array<string> = new Array("");
for (let char of "abcdefg") {
    var nextCombinations: Array<string> = new Array();
    for (let currentPattern of assignmentCombinations) {
        for (var index = 0; index <= currentPattern.length; index++) {
            let nextPattern = currentPattern.slice(0, index) + char + currentPattern.slice(index)
            nextCombinations.push(nextPattern);
        }
    }
    assignmentCombinations = nextCombinations;
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
for (let assignment of assignmentCombinations) {
    let displays = Array.from(digitToDisplay.values()).map(value => {
        return convert(value, DEFAULT_ASSIGNMENT, assignment);
    })
    let key = displays.sort().join(' ');
    assignmentIndex.set(key, assignment);
}

var part2 = 0;
for (let line of lines) {
    let [displays, values] = line.split(' | ')
    let key = displays.split(' ').map(display => {
        return Array.from(display).sort().join('')
    }).sort().join(" ")

    let assignment = assignmentIndex.get(key)!;
    let currentNumber = values.split(' ').reduce((sum, cur) => {
        let convertedDisplay = convert(cur, assignment, DEFAULT_ASSIGNMENT)
        return sum * 10 + displayToDigit.get(convertedDisplay)!
    }, 0);
    part2 += currentNumber;
}
print_result(8, 2, part2);
