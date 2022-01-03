import { readGrid, readStrings } from "./helpers";

let rows = readStrings(25).map(row => new Array(...row))
console.log(rows)

function move(previous: Array<Array<string>>): Array<Array<string>> {
    const height = previous.length
    const width = previous[0].length

    let next = new Array()
    for (let i =0; i < height; i++) {
        next.push(new Array(width).fill('.'))
    }

    // Move right
    for (let i = 0; i < height; i ++) {
        for (let j =0; j < width; j ++) {
            if (previous[i][j] == '>') {
                let right_j = (j + 1) % width
                if (previous[i][right_j] == '.') {
                    next[i][right_j] = '>'
                } else {
                    next[i][j] = '>'
                }
            }
        }
    }

    return next
}