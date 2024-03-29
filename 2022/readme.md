# Advent Of Code 2022

To run all days:

```
cargo run
```

To test for all days:

```
cargo test
```

To run tests for a specific day. For example, for day 10:

```
cargo test day10
```

To run tests with print output:

```
cargo test day10 -- --nocapture
```

To run a specific day, with animation (usually). For example, for day 14:

```
cargo run -- day14
```

## Day 7: No Space Left On Device

`RefCell<T>` and the Interior Mutability Pattern 
https://doc.rust-lang.org/book/ch15-05-interior-mutability.html

## Day 10: Cathode-Ray Tube

```
###..#....####.####.####.#.....##..####.
#..#.#....#.......#.#....#....#..#.#....
#..#.#....###....#..###..#....#....###..
###..#....#.....#...#....#....#.##.#....
#.#..#....#....#....#....#....#..#.#....
#..#.####.####.####.#....####..###.####.
```

## Day 13: Day 13: Distress Signal

https://doc.rust-lang.org/std/cmp/trait.Ord.html

## Day 14: Regolith Reservoir

```
$ cargo run -- day14
```

![](./2022day14-2.gif)

![](./2022day14.png)

## Day 16: Proboscidea Volcanium

https://www.reddit.com/r/adventofcode/comments/zn6k1l/2022_day_16_solutions/

Heavily inspired by discussion in reddit.

Solved with simple back-tracking and Floyd-Warshall algorithm. Without any heuristic optimization of early return, still able to produce result in about 45 seconds.

```
day16: this can take a while...
part1 time elapsed: 551.908516ms
part2 time elapsed: 44.178084961s
day16 part1: 2181
day16 part2: 2824
```
