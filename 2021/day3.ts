import { print_result, readStrings } from "./helpers";

let lines = readStrings(3);
var counters = new Array(lines[0].length).fill(0);
for (let line of lines) {
    for (let i = 0; i < line.length; i++) {
        if (line[i] == '1') {
            counters[i] += 1
        } else {
            counters[i] -= 1
        }
    }
}
console.log("counters", counters);

var gamma = 0;
var epsilon = 0;
for (let counter of counters) {
    gamma *= 2;
    epsilon *= 2;
    if (counter >= 0) {
        gamma += 1;
    } else if (counter < 0) {
        epsilon += 1;
    }
}

print_result(3, 1, gamma * epsilon)
