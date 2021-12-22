import { readStrings } from "./helpers";


class Pair {
    left: number | Pair
    right: number | Pair

    constructor(left: number | Pair, right: number | Pair) {
        this.left = left
        this.right = right
    }

    static parse(line: string): Pair {
        let stack = new Array()
        for (let ch of line.slice(0, line.length)) {
            switch (ch) {
                case "[":
                case ',':
                    break
                case ']':
                    let right = stack.pop()
                    let left = stack.pop()
                    stack.push(new Pair(left, right))
                    break
                default:
                    stack.push(parseInt(ch))
                    break
            }
        }
        return stack[stack.length - 1]
    }

    toString(): string {
        return `[${this.left.toString()},${this.right.toString()}]`
    }

    explode() {
        // TODO: how to explode this?
    }
}



// let lines = readStrings(18, '\n', "test1")
// let pairs = lines.map(line => {
//     let p = Pair.parse(line)
//     console.log(line)
//     console.log(p.toString())
//     return p
// })

let left = Pair.parse('[[[[4,3],4],4],[7,[[8,4],9]]]')
let right = Pair.parse('[1,1]')
let afterAddition = new Pair(left, right)
console.log(afterAddition.toString())