use crate::helpers::puzzle::Puzzle;
use crate::helpers::parser::read_single_string;

const IMAGE_WIDTH: usize = 25;
const IMAGE_HEIGHT: usize = 6;
const PIXELS_COUNT_PER_LAYER: usize = IMAGE_WIDTH * IMAGE_HEIGHT;
const NOT_INIT: char = '*';

pub struct Day8 {}

impl Puzzle<String, String> for Day8 {
    fn day(&self) -> i8 {
        8
    }

    fn parser(&self) -> fn(String) -> String {
        read_single_string
    }

    fn part1(&self, input: &String) -> String {
        let num_of_layers = input.len() / PIXELS_COUNT_PER_LAYER;

        let mut minimum_number_of_zeros = PIXELS_COUNT_PER_LAYER;
        let mut result = 0;
        for layer_index in 0..num_of_layers {
            let layer: String = input.chars()
                .skip(layer_index * PIXELS_COUNT_PER_LAYER)
                .take(PIXELS_COUNT_PER_LAYER)
                .collect();

            let zeros = layer.chars().filter(|x| x == &'0').count();
            let ones = layer.chars().filter(|x| x == &'1').count();
            let twos = layer.chars().filter(|x| x == &'2').count();

            if minimum_number_of_zeros > zeros {
                minimum_number_of_zeros = zeros;
                result = ones * twos;
            }
        }
        format!("{}", result)
    }

    fn part2(&self, input: &String) -> String {
        let num_of_layers = input.len() / PIXELS_COUNT_PER_LAYER;

        let mut visible_layer: Vec<char> = vec![NOT_INIT; PIXELS_COUNT_PER_LAYER];

        for layer_index in 0..num_of_layers {
            let layer: String = input.chars()
                .skip(layer_index * PIXELS_COUNT_PER_LAYER)
                .take(PIXELS_COUNT_PER_LAYER)
                .collect();

            for (i, c) in layer.chars().into_iter().enumerate() {
                if visible_layer[i] == NOT_INIT && c != '2' {
                    visible_layer[i] = c;
                }
            }
        }

        let mut result: String = String::new();
        result.push('\n');
        for j in 0..IMAGE_HEIGHT {
            for i in 0..IMAGE_WIDTH {
                if visible_layer[j * 25 + i] == '1' {
                    result.push_str("# ");
                } else {
                    result.push_str("  ");
                }
            }
            result.push('\n');
        }

        result
    }
}
