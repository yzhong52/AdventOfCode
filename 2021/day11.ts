import { colors, print_result, readGrid, sleep } from "./helpers";

let grid = readGrid(11);

let maxX = grid.length;
let maxY = grid[0].length;

let PADDING = '          '

function print_grid(title: string) {
    let formatted = grid.map(row => {
        return PADDING + row.map(num => {
            if (num == 0) {
                return `${colors.Reset}${colors.Bright}${colors.FgYellow}${colors.Bright}@`
            } else {
                return `${colors.Dim}${colors.FgWhite}${num}`
            }
        }).join(" ")
    }).join('\n')

    console.log(`\n ${colors.Reset}${colors.FgWhite}${colors.Bright}${title} \n\n${formatted} \n`);
}

async function main(display: boolean = false) {
    var part1_flashes = 0;
    var part2_step = 0;

    if (display) {
        print_grid(PADDING + "      Step 0");
        await sleep(5000);
    }


    function should_speed_up(step: number) {
        return step >= 15 && step < 350;
    }

    async function quick_display(step: number) {
        if (display) {
            await sleep(2);
            print_grid(PADDING + `      Step ${step}`);
        }
    }

    for (var step = 1; part2_step == 0; step++) {
        var step_counter = 0;

        let full: Array<[number, number]> = new Array();
        // First, the energy level of each octopus increases by 1.
        for (var x = 0; x < maxX; x++) {
            for (var y = 0; y < maxY; y++) {
                grid[x][y] += 1;

                if (grid[x][y] > 9) {
                    full.push([x, y])
                    grid[x][y] = 0
                    step_counter += 1
                }
                if (!should_speed_up(step)) {
                    await quick_display(step);
                }
            }
        }

        while (full.length) {
            let [x, y] = full.pop()!
            for (var x1 = Math.max(0, x - 1); x1 < Math.min(x + 2, maxX); x1++) {
                for (var y1 = Math.max(0, y - 1); y1 < Math.min(y + 2, maxY); y1++) {
                    if (grid[x1][y1] == 0) {  // Already flashed this round
                        continue
                    }
                    grid[x1][y1] += 1;
                    if (grid[x1][y1] > 9) {
                        full.push([x1, y1])
                        grid[x1][y1] = 0
                        step_counter += 1
                    }

                    if (!should_speed_up(step)) {
                        await quick_display(step);
                    }
                }
            }
        }

        if (display) {
            if (should_speed_up(step)) {
                if (step % 50 == 0) {
                    quick_display(step)
                    await sleep(250);
                }
            } else {
                quick_display(step)
                await sleep(250);
            }
        }

        if (step <= 100) {
            part1_flashes += step_counter;
        }
        if (step_counter == maxX * maxY) {
            part2_step = step;
        }

    }

    if (!display) {
        print_result(11, 1, part1_flashes)
        print_result(11, 2, part2_step)
    }
}

main(false)
