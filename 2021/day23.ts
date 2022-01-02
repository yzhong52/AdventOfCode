import PriorityQueue from "ts-priority-queue";
import { readStrings } from "./helpers";
import "./collections"

let rawInput = readStrings(23)

let wall: Set<string> = new Set()
let initialAmphipods: Map<string, Amphipod> = new Map()

type Pos = { y: number, x: number }

const seperator = '-'

function key(pos: Pos): string {
    return `${pos.y}${seperator}${pos.x}`
}

type AmphipodType = 'A' | 'B' | 'C' | 'D'

class Amphipod {
    constructor(public type: AmphipodType, public moves: number, public pos: Pos) { }

    key() {
        return key(this.pos)
    }
    get energy() {
        return Math.pow(10, this.type.charCodeAt(0) - 'A'.charCodeAt(0))
    }

    clone(props: {
        moves: number,
        pos: Pos
    }): Amphipod {
        return new Amphipod(
            this.type,
            props.moves,
            props.pos
        )
    }
}

for (let y = 0; y < rawInput.length; y++) {
    for (let x = 0; x < rawInput[y].length; x++) {
        let cell = rawInput[y][x]
        if (cell == '#') {
            wall.add(key({ y: y, x: x }))
        } else if (cell == '.' || cell == ' ') {
            continue
        } else {
            let amphipod = new Amphipod(cell as AmphipodType, 0, { x: x, y: y })
            initialAmphipods.set(amphipod.key(), amphipod)
        }
    }
}
let immediatelyOutsideRoomPos: Pos[] = [
    { y: 1, x: 3 },
    { y: 1, x: 5 },
    { y: 1, x: 7 },
    { y: 1, x: 9 },
]

let immediatelyOutsideRoomPosSet = new Set(immediatelyOutsideRoomPos.map(pos => key(pos)))
console.log(initialAmphipods)
console.log(immediatelyOutsideRoomPosSet)

let offsets = [[-1, 0], [1, 0], [0, -1], [0, 1]]

let targetPos = new Map<AmphipodType, Set<string>>()
for (let type of ['A', 'B', 'C', 'D']) {
    let x = 2 * (type.charCodeAt(0) - 'A'.charCodeAt(0)) + 3
    targetPos.set(
        type as AmphipodType,
        new Set([
            key({ y: 2, x: x }),
            key({ y: 3, x: x })
        ])
    )
}
console.log("Target Pos", targetPos)


class State {
    constructor(public energy: number, public amphipods: Map<string, Amphipod>) { }

    isFinal(): boolean {
        for (let [_, amphipod] of this.amphipods) {
            if (!targetPos.get(amphipod.type)!.has(amphipod.key())) {
                return false
            }
        }
        return true
    }

    clone(props: {
        energy: number,
        amphipods: Map<string, Amphipod>
    }): State {
        return new State(
            props.energy,
            props.amphipods
        )
    }

    key() {
        let buffer: Array<Array<string>> = new Array()
        for (let i = 0; i < rawInput.length; i++) {
            buffer.push(new Array(rawInput[i].length).fill(' '))
        }

        for (let amphipod of this.amphipods.values()) {
            buffer[amphipod.pos.y][amphipod.pos.x] = amphipod.type
        }

        return buffer.map(row => row.join('')).join('\n')
    }
}

function getPotentialMoves(movingAmphipod: Amphipod, amphipods: Map<string, Amphipod>): Array<[Pos, number]> {
    if (movingAmphipod.moves >= 2) {
        // Each amphipod can make no more than 2 moves. It will move
        // out of the room, and then move back to the right room.
        return []
    }

    let result: Array<[Pos, number]> = new Array()
    let visited: Set<string> = new Set()
    function dfs(current: Pos, distance: number) {
        let k = key(current)
        if (visited.has(k)) {
            return
        }
        if (wall.has(k)) {
            return
        }
        if (amphipods.has(k)) {
            return
        }
        visited.add(k)

        // cannot stop right outside of the room
        if (!immediatelyOutsideRoomPosSet.has(k)) {
            if (movingAmphipod.moves == 0) {
                // If we are moving outside, then y has to be 1
                if (current.y == 1) {
                    result.push([current, distance])
                }
            } else if (movingAmphipod.moves == 1) {
                // Amphipods will never move from the hallway into a room unless
                // 1) that room is their destination room and;
                // 2) that room contains no amphipods
                let isDestination = targetPos.get(movingAmphipod.type)!.has(k)

                let posBelow = key({ y: current.y + 1, x: current.x })

                if (
                    isDestination &&
                    (wall.has(posBelow) || amphipods.get(posBelow)?.type == movingAmphipod.type)
                ) {
                    result.push([current, distance])
                }
            }
        }
        for (let [dx, dy] of offsets) {
            dfs({ x: current.x + dx, y: current.y + dy }, distance + 1)
        }
    }

    for (let [dx, dy] of offsets) {
        dfs({ x: movingAmphipod.pos.x + dx, y: movingAmphipod.pos.y + dy }, 1)
    }

    return result
}

function debug(amphipods: Iterable<Amphipod>) {
    let buffer: Array<Array<string>> = new Array()
    for (let i = 0; i < rawInput.length; i++) {
        buffer.push(new Array(rawInput[i].length).fill(' '))
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

let initialState = new State(0, initialAmphipods)
let pq: PriorityQueue<State> = new PriorityQueue({
    comparator: (s1, s2) => s1.energy - s2.energy,
    initialValues: [initialState]
})
let seen: Set<string> = new Set()

let iteration = 0
while (pq.length != 0 && pq.length < 3620000) {
    iteration++
    let currentState = pq.dequeue()!
    if (seen.has(currentState.key())) {
        continue
    }
    seen.add(currentState.key())

    if (iteration % 50000 == 0 || currentState.isFinal()) {
        console.log("\nIteration:", iteration)
        console.log("Current energy:", currentState.energy)
        console.log("Total states:", pq.length)
        debug(currentState.amphipods.values())
    }

    if (currentState.isFinal()) {
        console.log(currentState.energy)
        break
    }

    for (let [_, amphipod] of currentState.amphipods) {
        let potentialMoves = getPotentialMoves(amphipod, currentState.amphipods)
        for (let [newPos, step] of potentialMoves) {
            let newAmphipod = amphipod.clone({
                pos: newPos,
                moves: amphipod.moves + 1
            })
            let newAmphipods = currentState.amphipods.clone()
            newAmphipods.delete(amphipod.key())
            newAmphipods.set(newAmphipod.key(), newAmphipod)
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
