import { print_result, readGrid } from "./helpers";

let grid = readGrid(9);

let maxX = grid.length;
let maxY = grid[0].length;

let offsets = [[-1, 0], [1, 0], [0, -1], [0, 1]];

function is_lower_point(x: number, y: number): boolean {
    for (let [dx, dy] of offsets) {
        let x1 = x + dx
        let y1 = y + dy
        if (x1 >= 0 && x1 < maxX && y1 >= 0 && y1 < maxY && grid[x1][y1] <= grid[x][y]) {
            return false
        }
    }
    return true
}

var part1 = 0

// This is also a strange behavior becaues of javascript
// https://stackoverflow.com/questions/2933737/when-iterating-over-values-why-does-typeofvalue-return-string-when-value-is
// `for (var i in grid) for (var j in grid[i])` won't work since i, j will be string type.
for (var i = 0; i < grid.length; i++) {
    for (var j = 0; j < grid[i].length; j++) {
        if (is_lower_point(i, j)) {
            part1 += grid[i][j] + 1
        }
    }
}
print_result(9, 1, part1);



function calculate_basin_size(x: number, y: number): number {
    if (grid[x][y] == 9) {
        return 0;
    }

    var counter = 1;
    var stack: Array<[number, number]> = new Array();
    stack.push([x, y]);
    grid[x][y] = 9;
    while (stack.length > 0) {
        let [x, y] = stack.pop()!;
        for (let [dx, dy] of offsets) {
            let x1 = x + dx;
            let y1 = y + dy;
            if (x1 >= 0 && x1 < maxX && y1 >= 0 && y1 < maxY && grid[x1][y1] < 9) {
                grid[x1][y1] = 9;
                stack.push([x1, y1]);
                counter += 1;
            }
        }
    }
    return counter;
}

var basin_sizes: Array<number> = new Array();
for (var i = 0; i < grid.length; i++) {
    for (var j = 0; j < grid[i].length; j++) {
        let basin_size = calculate_basin_size(i, j);
        if (basin_size != 0) {
            basin_sizes.push(basin_size);
        }
    }
}

// 1) No PriorityQueue in Typescript
// 2) We default sort in Typescript will sort it like this: [ 14, 3, 9, 9 ]
basin_sizes.sort((num1, num2) => num1 - num2);

let part2 = basin_sizes.slice(-3).reduce(((result, current) => result * current), 1);

print_result(9, 2, part2);
