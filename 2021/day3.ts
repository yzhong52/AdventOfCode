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
