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

People solve this with various ways: DFS, BFS, bitset, back-tracing, and even Floyd-Warshall algorithm. 

I haven't done much of those and simply using BFS here. 

But the biggest hint that help me through part 2 this is:

> I don't have to simulate both actors at once. One of us visits a certain subset of the valves, the other visits the 
> complement of that set. So it's just a matter of simulating visiting all possible subsets, then calculating the best 
> possible pressure of each complement pair of sets.

Otherwise, the complexity is too high.