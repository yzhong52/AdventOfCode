use super::super::helpers::parser::*;


pub fn part1(input: Input<(usize, i32)>) -> Answer<i32> {
    let (number_of_players, last_marble) = input.data;

    let mut marbles = vec![0, 2, 1];
    let mut current_number_index = 1;
    let mut player_id = 3;
    let mut scores: Vec<i32> = vec![0; number_of_players];

    for marble in 3..=last_marble {
        let index1 = (current_number_index + 1) % marbles.len();
        let index2 = (current_number_index + 2) % marbles.len();
        if marble % 23 == 0 {
            scores[player_id] += marble;

            let remove_marble_index = (current_number_index + marbles.len() - 7) % marbles.len();
            scores[player_id] += marbles[remove_marble_index];
            marbles.remove(remove_marble_index);
            current_number_index = remove_marble_index % marbles.len();
        } else {
            if index1 < index2 {
                marbles.insert(index2, marble);
                current_number_index = index2;
            } else {
                marbles.push(marble);
                current_number_index = marbles.len() - 1;
            }
        }

        player_id = (player_id + 1) % number_of_players;
    }

    Answer { question: input.question, result: *scores.iter().max().unwrap() }
}

pub fn part2(input: Input<(usize, i32)>) -> Answer<i32> {
    Answer { question: input.question, result: 0 }
}
