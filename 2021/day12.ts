import { print_result, readStrings } from "./helpers";
import { DefaultDict } from "./collections";
import "./clones";

let lines = readStrings(12)

let graph: DefaultDict<string, Array<string>> = new DefaultDict(() => new Array());
for (let row of lines) {
    let [fromNode, toNode] = row.split("-")
    graph.get(fromNode).push(toNode)
    graph.get(toNode).push(fromNode)
}

function countPath(canVisitOneSmallCaveTwice: boolean): number {
    let count: number = 0

    let stack: Array<[string, boolean, Set<string>]> = new Array();
    stack.push(['start', false, new Set()])

    while (stack.length > 0) {
        let [node, hasVisitedOneSmallCaveTwice, visitedSmallCaves] = stack.pop()!

        let isSmallCave: Boolean = node != "start" && node != "end" && node == node.toLocaleLowerCase();
        if (isSmallCave) {
            if (visitedSmallCaves.has(node)) {
                if (canVisitOneSmallCaveTwice && !hasVisitedOneSmallCaveTwice) {
                    hasVisitedOneSmallCaveTwice = true
                } else {
                    continue
                }
            } else {
                visitedSmallCaves.add(node)
            }
        }

        if (node == "end") {
            count += 1
        } else {
            for (let nextNode of graph.get(node)) {
                if (nextNode != "start") {
                    stack.push([nextNode, hasVisitedOneSmallCaveTwice, visitedSmallCaves.clone()])
                }
            }
        }
    }
    return count
}

let part1 = countPath(false)
print_result(12, 1, part1)

let part2 = countPath(true)
print_result(12, 2, part2)

