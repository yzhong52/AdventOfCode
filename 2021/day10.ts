import { print_result, readStrings } from "./helpers";

let lines = readStrings(10)

let close_to_open = new Map(
    [
        [')', '('],
        ['}', "{"],
        [']', "["],
        [">", "<"],
    ]
)
let open_to_close = new Map(
    [
        ['(', ')'],
        ["{", '}'],
        ["[", ']'],
        ["<", '>'],
    ]
)

let part1_scores = new Map(
    [
        [")", 3],
        ["]", 57],
        ["}", 1197],
        [">", 25137],
    ]
)

let part2_scores: Map<string, number> = new Map(
    [
        [")", 1],
        ["]", 2],
        ["}", 3],
        [">", 4],
    ]
)

var part1: number = 0
var part2s: Array<number> = new Array();
for (let line of lines) {
    var stack: Array<string> = new Array();
    var isInvalid = false;
    for (var i = 0; i < line.length; i += 1) {
        let char = line[i];
        if (close_to_open.has(char)) {
            let open = close_to_open.get(char)!
            if (stack.length > 0 && stack.pop() == open) {
            } else {
                // This is a invalid char 
                part1 += part1_scores.get(char)!
                isInvalid = true;
                break;
            }
        } else {
            stack.push(char);
        }
    }

    if (!isInvalid) {
        var score: number = 0;
        while (stack.length > 0) {
            let open = stack.pop()!;
            let close = open_to_close.get(open)!
            score = score * 5 + part2_scores.get(close)!;
        }
        part2s.push(score);
    }

}
print_result(10, 1, part1);

// Sorting integers in Typescript is so weird.
part2s.sort((num1, num2) => num1 - num2);

print_result(10, 2, part2s[Math.floor(part2s.length / 2)]);