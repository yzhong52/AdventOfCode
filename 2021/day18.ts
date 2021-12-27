import { assert } from "console";
import { assertEqual } from "./utils/asserts";
import { print_result, readStrings } from "./helpers";

class Pair {
    constructor(
        public left: Node,
        public right: Node
    ) { }

    toString() {
        return `[${this.left.toString()},${this.right.toString()}]`
    }

    clone() {
        return new Pair(this.left.clone(), this.right.clone())
    }
}

class Node {

    constructor(public value: number | Pair) {
    }

    static createPair(left: Node, right: Node): Node {
        return new Node(new Pair(left, right))
    }

    getValue(): number | undefined {
        if (typeof this.value === 'number') {
            return this.value as number
        }
        return undefined
    }

    getChildren(): Pair | undefined {
        if (typeof this.value === 'number') {
            return undefined
        }
        return this.value as Pair
    }

    static parse(line: string): Node {
        let stack: Array<Node> = new Array()
        for (let ch of line.slice(0, line.length)) {
            switch (ch) {
                case "[":
                case ',':
                    break
                case ']':
                    let right = stack.pop()
                    let left = stack.pop()
                    stack.push(Node.createPair(left!, right!))
                    break
                default:
                    stack.push(new Node(parseInt(ch)))
                    break
            }
        }
        return stack[stack.length - 1]
    }

    toString(): string {
        return this.value.toString()
    }

    tryExplode() {
        var exploded = false
        var previousLeafNode: Node | undefined = undefined
        var pendingRightNumber: number | undefined = undefined

        function visit(depth: number, currentNode: Node) {
            if (typeof currentNode.value === 'number') {
                previousLeafNode = currentNode
                if (pendingRightNumber) {
                    currentNode.value = pendingRightNumber + currentNode.getValue()!
                    pendingRightNumber = undefined
                }
            } else {
                let children = currentNode.getChildren()!
                if (depth < 4 || exploded) { // We only explode one number per round
                    visit(depth + 1, children.left)
                    visit(depth + 1, children.right)
                } else {
                    let currentLeftNumber: number | undefined = children.left.getValue()
                    let currentRightNumber: number | undefined = children.right.getValue()

                    if (currentLeftNumber != undefined && currentRightNumber != undefined) {
                        exploded = true
                        if (previousLeafNode) {
                            assert(typeof previousLeafNode.value === 'number')
                            previousLeafNode.value = currentLeftNumber + previousLeafNode.getValue()!
                        }
                        pendingRightNumber = currentRightNumber
                        previousLeafNode = currentNode
                        currentNode.value = 0
                    } else {
                        visit(depth + 1, children.left)
                        visit(depth + 1, children.right)
                    }
                }
            }
        }
        visit(0, this)
        return exploded
    }

    explode() {
        this.tryExplode()
        return this
    }

    trySplit() {
        var hasSplit = false
        function visit(node: Node) {
            if (hasSplit) {
                // Only split one node at a time
                return
            }
            if (node.getValue() != undefined) {
                let value = node.getValue()!
                if (value >= 10) {
                    hasSplit = true
                    node.value = new Pair(
                        new Node(Math.floor(value / 2)),
                        new Node(Math.ceil(value / 2))
                    )
                }
            } else {
                let child = node.getChildren()!
                visit(child.left)
                visit(child.right)
            }
        }
        visit(this)
        return hasSplit
    }

    split() {
        this.trySplit()
        return this
    }

    static add(node1: Node, node2: Node): Node {
        return new Node(new Pair(node1.clone(), node2.clone()))
    }

    clone(): Node {
        if (this.getValue() != undefined) {
            return new Node(this.getValue()!)
        } else {
            let children = this.getChildren()!
            return new Node(children.clone())
        }
    }

    reduce(): Node {
        while (true) {
            // During reduction, at most one action applies, after which the process returns
            // to the top of the list of actions. For example, if split produces a pair that
            // meets the explode criteria, that pair explodes before other splits occur.
            if (this.tryExplode()) {
                continue
            }
            if (this.trySplit()) {
                continue
            }
            break
        }
        return this
    }

    magnitude(): number {
        if (this.getValue() != undefined) {
            return this.getValue()!
        } else {
            return 3 * this.getChildren()?.left.magnitude()! + 2 * this.getChildren()?.right.magnitude()!
        }
    }

    static sum(nodes: Array<Node>): Node {
        return nodes
            .slice(1)
            .reduce((previous, current) => Node.add(previous, current).reduce(), nodes[0])
    }
}


assertEqual(
    Node.parse('[[[[[9,8],1],2],3],4]').explode().toString(),
    "[[[[0,9],2],3],4]"
)

assertEqual(
    Node.parse('[7,[6,[5,[4,[3,2]]]]]').explode().toString(),
    "[7,[6,[5,[7,0]]]]"
)

assertEqual(
    Node.parse('[[6,[5,[4,[3,2]]]],1]').explode().toString(),
    "[[6,[5,[7,0]]],3]"
)

assertEqual(
    Node.parse('[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]').reduce().toString(),
    "[[3,[2,[8,0]]],[9,[5,[7,0]]]]"
)

assertEqual(
    Node.parse('[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]').reduce().toString(),
    "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"
)

let test1_to_6 = `[1,1]
[2,2]
[3,3]
[4,4]
[5,5]
[6,6]
`.trim().split('\n')
assertEqual(
    Node.sum(test1_to_6.map(row => Node.parse(row))).toString(),
    '[[[[5,0],[7,4]],[5,5]],[6,6]]'
)


assertEqual(
    Node.add(
        Node.parse('[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]'),
        Node.parse('[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]')
    )
        .reduce().toString(),
    "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]"
)

let test_larger = `[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
[7,[5,[[3,8],[1,4]]]]
[[2,[2,2]],[8,[8,1]]]
[2,9]
[1,[[[9,3],9],[[9,0],[0,7]]]]
[[[5,[7,4]],7],1]
[[[[4,2],2],6],[8,7]]
`.trim().split('\n')
assertEqual(
    Node.sum(test_larger.map(row => Node.parse(row))).toString(),
    '[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]'
)


let rows = readStrings(18)
let nodes = rows.map(row => Node.parse(row))
print_result(18, 1, Node.sum(nodes).magnitude())


let part2 = 0
for (let i = 0; i < nodes.length; i++) {
    for (let j = 0; j < nodes.length; j++) {
        if (i != j) {
            let magnitude = Node.add(nodes[i], nodes[j]).reduce().magnitude()
            part2 = Math.max(part2, magnitude)
        }
    }
}
print_result(18, 2, part2)