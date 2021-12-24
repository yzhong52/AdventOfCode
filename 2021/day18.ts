import { assert } from "console";
import { assertEqual } from "./utils/asserts";
import { readStrings } from "./helpers";

class Pair {
    constructor(
        public left: Node,
        public right: Node
    ) { }

    toString() {
        return `[${this.left.toString()},${this.right.toString()}]`
    }
}

class Node {

    constructor(public value: number | Pair) {
    }

    static createPair(left: Node, right: Node) {
        return new Node(new Pair(left, right))
    }

    get left() {
        return (this.value as Pair)?.left
    }

    get right() {
        return (this.value as Pair)?.right
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
        var previousNode: Node | undefined = undefined
        var rightNumber: number | undefined = undefined

        function visit(depth: number, currentNode: Node) {
            if (typeof currentNode.value === 'number') {
                previousNode = currentNode
                if (rightNumber) {
                    currentNode.value = rightNumber + (currentNode.value as number)
                    rightNumber = undefined
                }
                return
            } else {
                let value = currentNode.value as Pair
                if (depth < 4) {
                    visit(depth + 1, value.left)
                    visit(depth + 1, value.right)
                } else {
                    exploded = true
                    assert(depth == 4)
                    let leftNumber = (currentNode.left.value as number)
                    if (rightNumber) {
                        leftNumber += rightNumber
                    }
                    rightNumber = (currentNode.right.value as number)
                    if (previousNode) {
                        previousNode.value = leftNumber + (previousNode.value as number)
                    }
                    previousNode = currentNode
                    currentNode.value = 0
                }
            }
        }
        visit(0, this)

        if (exploded) {
            console.log("e", this.toString())
        }
        return exploded
    }

    explode() {
        this.tryExplode()
        return this
    }

    trySplit() {
        var hasSplit = false
        function visit(node: Node) {
            if (typeof node.value === 'number') {
                if (node.value >= 10) {
                    hasSplit = true
                    node.value = new Pair(
                        new Node(Math.floor(node.value / 2)),
                        new Node(Math.ceil(node.value / 2))
                    )
                }
            } else {
                visit(node.left)
                visit(node.right)
            }
        }
        visit(this)

        if (hasSplit) {
            console.log("s", this.toString())
        }
        return hasSplit
    }

    split() {
        this.trySplit()
        return this
    }

    add(node: Node) {
        console.log(" ", this.toString())
        console.log("+", node.toString())
        let result = new Node(new Pair(this, node))
        console.log("=", result.toString(), "\n")
        return result
    }

    reduce() {
        while (this.tryExplode() || this.trySplit()) {

        }
        return this
    }

    static sum(rows: Array<string>): Node {
        let nodes = rows.map(row => Node.parse(row))
        return nodes
            .slice(1)
            .reduce((previous, current) => previous.add(current).reduce(), nodes[0])
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
    Node.parse('[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]').explode().toString(),
    "[[3,[2,[8,0]]],[9,[5,[7,0]]]]"
)

assertEqual(
    Node.parse('[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]').explode().split().explode().toString(),
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
    Node.sum(test1_to_6).toString(),
    '[[[[5,0],[7,4]],[5,5]],[6,6]]'
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
    Node.sum(test_larger).toString(),
    '[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]'
)


// let testNodeLeft = Node.parse('[[[[4,3],4],4],[7,[[8,4],9]]]')
// let testNodeRight = Node.parse('[1,1]')
// let node = new Node(new Pair(testNodeLeft, testNodeRight))
// console.log("after addition:", node.toString())
// node.explode()
// console.log("after explode:", node.toString())

