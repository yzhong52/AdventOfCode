use super::super::helpers::parser::*;
use std::collections::HashMap;

const TARGET_MATERIAL: &str = "FUEL";
const SOURCE_MATERIAL: &str = "ORE";
const MAXIMUM_ORE: i64 = 1000000000000;

#[derive(Debug)]
struct RequiredMaterial {
    material: String,
    quantity: i64,
}

impl RequiredMaterial {
    fn new(raw: &str) -> RequiredMaterial {
        let material_with_quantity: Vec<&str> = raw.split(' ').collect();
        RequiredMaterial {
            material: material_with_quantity[1].to_string(),
            quantity: material_with_quantity[0].parse::<i64>().unwrap(),
        }
    }
}

fn parse_reactions(data: &Vec<String>) -> HashMap<String, (Vec<RequiredMaterial>, i64)> {
    let mut reactions: HashMap<String, (Vec<RequiredMaterial>, i64)> = HashMap::new();

    for row in data {
        let components: Vec<&str> = row.split(" => ").collect();
        let input_materials: Vec<RequiredMaterial> = components[0].split(", ").map(RequiredMaterial::new).collect();
        let output_material = RequiredMaterial::new(components[1]);
        assert!(!reactions.contains_key(&output_material.material), "There are multiple ways to generate one material?!");
        reactions.insert(output_material.material, (input_materials, output_material.quantity));
    }
    reactions
}


// `required_materials`: Negative value indicates that we have some remained from earlier reactions
fn run_simulation(
    reactions: &HashMap<String, (Vec<RequiredMaterial>, i64)>,
    required_materials: &mut HashMap<String, i64>) -> i64 {
    let mut next_ups = vec![TARGET_MATERIAL.to_string()];

    while next_ups.len() > 0 {
        let next_up = next_ups.pop().unwrap();

        let (input_materials, output_quality) = reactions.get(&next_up).unwrap();

        let required_quality = *required_materials.get(&next_up).unwrap();
        if required_quality > 0 {
            let scale = required_quality / *output_quality + (required_quality % *output_quality != 0) as i64;

            required_materials.insert(next_up.clone(), required_quality - scale * *output_quality);

            for input in input_materials {
                let required_quantity = required_materials.get(&input.material).unwrap_or(&0) + input.quantity * scale;

                if required_quantity > 0 && input.material != SOURCE_MATERIAL.to_string() {
                    next_ups.push(input.material.clone());
                }
                required_materials.insert(input.material.clone(), required_quantity);
            }
        }
    }

    required_materials[&SOURCE_MATERIAL.to_string()]
}

fn required_ore_for_one(reactions: &HashMap<String, (Vec<RequiredMaterial>, i64)> ) -> i64 {
    let mut required_materials: HashMap<String, i64> = HashMap::new();
    required_materials.insert(TARGET_MATERIAL.to_string(), 1);

    run_simulation(&reactions, &mut required_materials)
}

pub fn part1(input: Input<Vec<String>>) -> Answer<i64> {
    let reactions: HashMap<String, (Vec<RequiredMaterial>, i64)> = parse_reactions(&input.data);
    let required_ore = required_ore_for_one(&reactions);
    return Answer { question: input.question, result: required_ore };
}

pub fn part2(input: Input<Vec<String>>) -> Answer<i64> {
    let reactions = parse_reactions(&input.data);

    let single_fuel_required_ore = required_ore_for_one(&reactions);

    // Negative value indicates that we have some remained from earlier reactions
    let mut required_materials: HashMap<String, i64> = HashMap::new();

    let mut current_used_ore: i64 = 0;
    let mut loop_index = 0;
    loop {
        let remaining_ore = MAXIMUM_ORE - current_used_ore;

        // With `remaining_ore` ORE, we can approximately produce
        let target_quantity = i64::max(remaining_ore / single_fuel_required_ore / 2 - 1, 1);

        required_materials.insert(TARGET_MATERIAL.to_string(), target_quantity);

        let new_required_ore = run_simulation(&reactions, &mut required_materials);
        if new_required_ore > MAXIMUM_ORE {
            break;
        } else {
            current_used_ore = new_required_ore;
        }

        loop_index += 1;
        if loop_index % 1000 == 0 {
            println!("{}/{} about {}", current_used_ore, MAXIMUM_ORE, current_used_ore as f32 / MAXIMUM_ORE as f32);
        }
    }

    return Answer { question: input.question, result: current_used_ore };
}
