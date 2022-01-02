import PriorityQueue from "ts-priority-queue";
import { print_result, readStrings } from "./helpers";
import "./collections"

const seperator = '-'

class Pos {
    y: number
    x: number

    constructor(pos: { y: number, x: number }) {
        this.y = pos.y
        this.x = pos.x
    }

    get isInRoom() {
        return this.y >= 2
    }

    get key() {
        return `${this.y}${seperator}${this.x}`
    }

    add(pos: { y: number, x: number }): Pos {
        return new Pos({ y: this.y + pos.y, x: this.x + pos.x })
    }
}


type AmphipodType = 'A' | 'B' | 'C' | 'D'

class Amphipod {
    constructor(public type: AmphipodType, public moves: number, public pos: Pos) { }

    get key() {
        return this.pos.key
    }

    get energy() {
        return Math.pow(10, this.type.charCodeAt(0) - 'A'.charCodeAt(0))
    }

    clone(props: { moves: number, pos: Pos }): Amphipod {
        return new Amphipod(
            this.type,
            props.moves,
            props.pos
        )
    }

    targetX() {
        return 2 * (this.type.charCodeAt(0) - 'A'.charCodeAt(0)) + 3
    }

    isInTargetRoom(): boolean {
        return this.pos.x == this.targetX() && this.pos.isInRoom
    }
}

const immediatelyOutsideRoomPosSet = new Set([
    new Pos({ y: 1, x: 3 }).key,
    new Pos({ y: 1, x: 5 }).key,
    new Pos({ y: 1, x: 7 }).key,
    new Pos({ y: 1, x: 9 }).key,
])

const offsets = [[-1, 0], [1, 0], [0, -1], [0, 1]]

class State {
    constructor(public energy: number, public amphipods: Map<string, Amphipod>) { }

    isSatisfied(): boolean {
        return new Array(...this.amphipods.values())
            .every(amphipod => amphipod.isInTargetRoom())
    }

    clone(props: { energy: number, amphipods: Map<string, Amphipod> }): State {
        return new State(props.energy, props.amphipods)
    }

    key() {
        let amphipods = new Array(...this.amphipods.values())
        let maxY = Math.max(...amphipods.map(amphipod => amphipod.pos.y)) + 1
        let maxX = Math.max(...amphipods.map(amphipod => amphipod.pos.x)) + 1

        let buffer: Array<Array<string>> = new Array()
        for (let i = 0; i < maxY; i++) {
            buffer.push(new Array(maxX).fill(' '))
        }
        for (let amphipod of amphipods) {
            buffer[amphipod.pos.y][amphipod.pos.x] = amphipod.type
        }

        return buffer.map(row => row.join('')).join('\n')
    }
}

function getPotentialMoves(
    movingAmphipod: Amphipod,
    amphipods: Map<string, Amphipod>,
    wall: Set<string>
): Array<[Pos, number]> {
    if (movingAmphipod.moves >= 2) {
        // Each amphipod can make no more than 2 moves. It will move
        // out of the room, and then move back to the right room.
        return []
    }

    let result: Array<[Pos, number]> = new Array()
    let visited: Set<string> = new Set()

    function dfs(current: Pos, distance: number) {
        if (visited.has(current.key)) {
            return
        }
        if (wall.has(current.key)) {
            return
        }
        if (amphipods.has(current.key)) {
            return
        }
        visited.add(current.key)

        // Cannot stop right outside of the room
        if (!immediatelyOutsideRoomPosSet.has(current.key)) {
            if (movingAmphipod.moves == 0) {
                // If we are moving outside, then y has to be 1
                if (!current.isInRoom) {
                    result.push([current, distance])
                }
            } else if (movingAmphipod.moves == 1) {
                // Amphipods will never move from the hallway into a room unless
                // 1) that room is their destination room and;
                // 2) that room contains no amphipods
                let posBelow = current.add({ y: 1, x: 0 }).key
                let hasNoConflicts = wall.has(posBelow) || amphipods.get(posBelow)?.type == movingAmphipod.type
                if (movingAmphipod.targetX() == current.x && current.isInRoom && hasNoConflicts) {
                    result.push([current, distance])
                }
            }
        }
        for (let [dx, dy] of offsets) {
            dfs(current.add({ x: dx, y: dy }), distance + 1)
        }
    }

    for (let [dx, dy] of offsets) {
        dfs(movingAmphipod.pos.add({ x: dx, y: dy }), 1)
    }

    return result
}

function debug(amphipods: Iterable<Amphipod>, wall: Set<string>, height: number, width: number) {
    let buffer: Array<Array<string>> = new Array()

    for (let i = 0; i < height; i++) {
        buffer.push(new Array(width).fill(' '))
    }

    for (let w of wall) {
        let [y, x] = w.split(seperator).map(value => parseInt(value))
        buffer[y][x] = '#'
    }

    for (let amphipod of amphipods) {
        buffer[amphipod.pos.y][amphipod.pos.x] = amphipod.type
    }

    console.log(buffer.map(row => row.join('')).join('\n'))
}

function parse(rawInput: Array<string>): [Map<string, Amphipod>, Set<string>] {
    let wall: Set<string> = new Set()
    let initialAmphipods: Map<string, Amphipod> = new Map()

    for (let y = 0; y < rawInput.length; y++) {
        for (let x = 0; x < rawInput[y].length; x++) {
            let cell = rawInput[y][x]
            let cellPos = new Pos({ y: y, x: x })
            if (cell == '#') {
                wall.add(cellPos.key)
            } else if (cell == '.' || cell == ' ') {
                continue
            } else {
                let amphipod = new Amphipod(cell as AmphipodType, 0, cellPos)
                initialAmphipods.set(amphipod.key, amphipod)
            }
        }
    }
    return [initialAmphipods, wall]
}

function run(rawInput: Array<string>) {
    let [initialAmphipods, wall] = parse(rawInput)

    let initialState = new State(0, initialAmphipods)
    let pq: PriorityQueue<State> = new PriorityQueue({
        comparator: (s1, s2) => s1.energy - s2.energy,
        initialValues: [initialState]
    })
    let seen: Set<string> = new Set()

    let iteration = 0
    let result = -1
    while (pq.length != 0) {
        iteration++
        let currentState = pq.dequeue()!
        if (seen.has(currentState.key())) {
            continue
        }
        seen.add(currentState.key())

        if (iteration % 500 == 0 || currentState.isSatisfied()) {
            console.log("\nIteration:", iteration)
            console.log("Current energy:", currentState.energy)
            console.log("Total states:", pq.length)
            debug(currentState.amphipods.values(), wall, rawInput.length, rawInput[0].length)
        }

        if (currentState.isSatisfied()) {
            result = currentState.energy
            break
        }

        for (let [_, amphipod] of currentState.amphipods) {
            let potentialMoves = getPotentialMoves(amphipod, currentState.amphipods, wall)
            for (let [newPos, step] of potentialMoves) {
                let newAmphipod = amphipod.clone({
                    pos: newPos,
                    moves: amphipod.moves + 1
                })
                let newAmphipods = currentState.amphipods.clone()
                newAmphipods.delete(amphipod.key)
                newAmphipods.set(newAmphipod.key, newAmphipod)
                let newState = new State(
                    step * amphipod.energy + currentState.energy,
                    newAmphipods
                )
                if (!seen.has(newState.key())) {
                    pq.queue(newState)
                }
            }
        }
    }
    return result
}

let part1Input: string[] = readStrings(23)
let part1 = run(part1Input)
print_result(23, 1, part1)

let part2Input: string[] = part1Input.slice(0, 3)
    .concat("  #D#C#B#A#", "  #D#B#A#C#")
    .concat(...part1Input.slice(3))
console.log(part2Input)
let part2 = run(part2Input)
print_result(23, 2, part2)
