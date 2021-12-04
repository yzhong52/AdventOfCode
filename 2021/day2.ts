import * as fs from 'fs';

let lines: string[] = fs.readFileSync('day2', 'utf8').trim().split('\n');

var horizontal = 0
var depth = 0
for (let line of lines) {
    let words = line.split(' ');
    let direction = words[0];
    let num = parseInt(words[1]);
    switch (direction) {
        case 'forward':
            horizontal += num;
            break;
        case 'backward':
            horizontal -= num;
            break;
        case 'up':
            depth -= num;
            break;
        case 'down':
            depth += num;
            break;
        default:
            break;
    }
}

console.log("Part 1", horizontal * depth);

var horizontal = 0
var depth = 0
var aim = 0
for (let line of lines) {
    let words = line.split(' ');
    let direction = words[0];
    let num = parseInt(words[1]);
    switch (direction) {
        case 'forward':
            horizontal += num;
            depth += aim * num;
            break;
        case 'backward':
            horizontal -= num;
            break;
        case 'up':
            aim -= num;
            break;
        case 'down':
            aim += num;
            break;
        default:
            break;
    }
}

console.log("Part 2", horizontal * depth);
