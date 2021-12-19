import { Counter } from "./collections";
import { print_result, readStrings } from "./helpers";
import "./utils/maps";
import "./utils/foreach";

let lines = readStrings(14)
let polymerTemplate = lines[0]
let pairInsertionRules = lines.slice(2)

let rules: Map<string, string> = pairInsertionRules.map(rule => rule.split(' -> ') as [string, string]).toMap()

function insert(pairCounts: Map<string, number>): Counter<string> {
    let result = new Counter<string>()
    for (let [pair, count] of pairCounts) {
        let insertChar = rules.get(pair)
        if (insertChar) {
            result.add(pair[0] + insertChar, count)
            result.add(insertChar + pair[1], count)
        } else {
            result.add(pair, count)
        }
    }
    return result
}

let initialPairCounts = new Counter<string>();
for (var i = 0; i < polymerTemplate.length - 1; i ++) {
    let key: string = polymerTemplate.slice(i, i + 2)
    initialPairCounts.add(key, 1)
}

function countElements(pairCounter: Counter<string>) {
    let counter = new Counter<string>()
    for (let [pair, count] of pairCounter) {
        pair.forEach(element => {
            counter.add(element, count)
        });
    }
    counter.add(polymerTemplate[0], 1)
    counter.add(polymerTemplate[polymerTemplate.length - 1], 1)
    for (let [key, count] of counter) {
        counter.set(key, count / 2)
    }
    return counter
}

function calculate(step: number): number {
    var pairCount = initialPairCounts;
    for (var i = 0; i < step; i++) {
        pairCount = insert(pairCount)
    }
    let counts = countElements(pairCount)
    return Math.max(...counts.values()) - Math.min(...counts.values())
}

print_result(14, 1, calculate(10))
print_result(14, 2, calculate(40))
