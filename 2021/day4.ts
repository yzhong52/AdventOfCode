import { print_result, readStrings } from "./helpers";

const BOARD_SIZE = 5;
let lines = readStrings(4)

function readBoard(index: number) {
    let rows = lines.slice(index, index + 5)
    return rows.map(row => {
        // '/\s+/' here is for splitting by multiple spaces
        // https://stackoverflow.com/a/40282581/1035008
        return row.trim().split(/\s+/).flatMap(value => {
            return parseInt(value);
        })
    })
}

class Board {
    grid: Array<Array<number>>;
    rowCounter: Array<number> = new Array<number>(BOARD_SIZE).fill(0);
    colCounter: Array<number> = new Array<number>(BOARD_SIZE).fill(0);
    grid_index: Map<number, [number, number]>;
    private _has_won: boolean = false;

    constructor(grid: Array<Array<number>>) {
        this.grid = grid;
        this.grid_index = new Map();
        for (let i = 0; i < BOARD_SIZE; i += 1) {
            for (let j = 0; j < BOARD_SIZE; j += 1) {
                this.grid_index.set(grid[i][j], [i, j]);
            }
        }
    }

    mark(num: number) {
        if (!this.grid_index.has(num)) {
            return
        }

        let index: [number, number] = this.grid_index.get(num)!;
        this.grid_index.delete(num);

        this.rowCounter[index[0]] += 1;
        this.colCounter[index[1]] += 1;
    }

    public get has_won(): boolean {
        if (this._has_won) {
            return this._has_won;
        }

        // O(N) - can be improved to O(1), but since BOARD_SIZE is only 5, it doesn't matter.
        this._has_won = this.rowCounter.includes(BOARD_SIZE) || this.colCounter.includes(BOARD_SIZE);
        return this._has_won;
    }

    sum_all_unmarked_numbers() {
        var sum = 0
        for (let key of this.grid_index.keys()) {
            sum += key
        }
        return sum;
    }

}

let boards: Array<Board> = new Array();
var index = 2;
while (index + BOARD_SIZE <= lines.length) {
    boards.push(new Board(readBoard(index)));
    // There is an empty line after each board
    index += BOARD_SIZE + 1;
}

let drawn_numbers = lines[0].split(',').map(value => parseInt(value))

var winnerCount = 0;
for (let num of drawn_numbers) {
    for (let bid = 0; bid < boards.length; bid += 1) {
        let board = boards[bid];
        if (board.has_won) {
            continue;
        }

        board.mark(num)
        if (board.has_won) {
            winnerCount += 1;
            if (winnerCount == 1) {
                console.log(`Board ${bid} is wining at number ${num}`);
                print_result(4, 1, board.sum_all_unmarked_numbers() * num)
            }
            if (winnerCount == boards.length) {
                console.log(`Board ${bid} is finally wining at number ${num}`);
                print_result(4, 2, board.sum_all_unmarked_numbers() * num)
            }
        }
    }
}
