import { print_result, readGrid, readStrings, sleep } from "./helpers";

function move(previous: Array<Array<string>>): [Array<Array<string>>, boolean] {
    const height = previous.length
    const width = previous[0].length

    let next = new Array()
    for (let i = 0; i < height; i++) {
        next.push(new Array(width).fill('.'))
    }

    let moved = false

    // Move right
    for (let i = 0; i < height; i++) {
        for (let j = 0; j < width; j++) {
            if (previous[i][j] == '>') {
                let right_j = (j + 1) % width
                if (previous[i][right_j] == '.') {
                    next[i][right_j] = '>'
                    moved = true
                } else {
                    next[i][j] = '>'
                }
            }
        }
    }

    // Move down
    for (let i = 0; i < height; i++) {
        for (let j = 0; j < width; j++) {
            if (previous[i][j] == 'v') {
                let bottom_i = (i + 1) % height
                if (previous[bottom_i][j] != 'v' && next[bottom_i][j] != '>') {
                    next[bottom_i][j] = 'v'
                    moved = true
                } else {
                    next[i][j] = 'v'
                }
            }
        }
    }

    return [next, moved]
}

function show(step: number, arr: Array<Array<string>>) {
    let cucumbers = arr.map(row => row.join('')).join('\n')
    let buffer = `After ${step} step:\n${cucumbers}\n`
    console.log(buffer)
}



var rows = readStrings(25).map(row => new Array(...row))

async function main(display: boolean) {
    let i = 0
    let moved = true
    if (display) {
        show(i, rows)
    }
    while (moved) {
        i++
        [rows, moved] = move(rows)
        if (display) {
            show(i, rows)
            await sleep(50)
        }
    }
    print_result(25, 1, i)
}

main(true)
