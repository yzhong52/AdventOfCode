use super::super::helpers::parser::*;
use super::super::helpers::models::*;

pub fn parse_points(data: &Vec<String>) -> Vec<Point3D> {
    let mut points: Vec<Point3D> = vec![];
    for row in data {
        // Remove '<' and '>' from "<x=-5, y=4, z=9>".
        let sub_str = &row[1..row.len() - 1];

        let components: Vec<&str> = sub_str.split(',').map(|x| x.split('=').last().unwrap()).collect();

        let point = Point3D {
            x: components[0].parse::<i32>().unwrap(),
            y: components[1].parse::<i32>().unwrap(),
            z: components[2].parse::<i32>().unwrap(),
        };

        points.push(point);
    }
    points
}

fn adjust(left: i32, right: i32) -> i32 {
    if left == right {
        0
    } else {
        (left - right) / (left - right).abs()
    }
}

fn update_velocities(points: &Vec<Point3D>, velocities: &mut Vec<Point3D>) {
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            velocities[i].x += adjust(points[j].x, points[i].x);
            velocities[i].y += adjust(points[j].y, points[i].y);
            velocities[i].z += adjust(points[j].z, points[i].z);

            velocities[j].x += adjust(points[i].x, points[j].x);
            velocities[j].y += adjust(points[i].y, points[j].y);
            velocities[j].z += adjust(points[i].z, points[j].z);
        }
    }
}

fn update_positions(points: &mut Vec<Point3D>, velocities: &Vec<Point3D>) {
    for i in 0..points.len() {
        points[i].x += velocities[i].x;
        points[i].y += velocities[i].y;
        points[i].z += velocities[i].z;
    }
}

pub fn part1(input: Input<Vec<String>>) -> Answer<i32> {
    let mut points: Vec<Point3D> = parse_points(&input.data);
    let mut velocities: Vec<Point3D> = vec![Point3D::origin(); points.len()];

    let mut total_energy: i32 = 0;
    for _ in 0..1000 {
        update_velocities(&points, velocities.as_mut());
        update_positions(points.as_mut(), &velocities);

        total_energy = 0;
        for i in 0..points.len() {
            let kinetic_energy = velocities[i].x.abs() + velocities[i].y.abs() + velocities[i].z.abs();
            let potential_energy = points[i].x.abs() + points[i].y.abs() + points[i].z.abs();
            total_energy += kinetic_energy * potential_energy
        }
    }
    return Answer { question: input.question, result: total_energy };
}


pub fn part2(input: Input<Vec<String>>) -> Answer<usize> {
    let initial_points: Vec<Point3D> = parse_points(&input.data);

    fn to_x(p: &Point3D) -> i32 { p.x }
    fn to_y(p: &Point3D) -> i32 { p.y }
    fn to_z(p: &Point3D) -> i32 { p.z }

    let projections: [fn(&Point3D) -> i32; 3] = [to_x, to_y, to_z];

    let dimensional_repeat_counts: Vec<usize> = projections.iter().map(|projection_function| {
        // TODO: Shouldn't this start with 0?
        let mut index: usize = 1;
        let mut points: Vec<Point3D> = initial_points.clone();
        let mut velocities: Vec<Point3D> = vec![Point3D::origin(); points.len()];

        loop {
            update_velocities(&points, velocities.as_mut());
            update_positions(points.as_mut(), &velocities);
            index += 1;

            if initial_points.iter().map(projection_function).collect::<Vec<i32>>() == points.iter().map(projection_function).collect::<Vec<i32>>() {
                break;
            }
        }
        index
    }).collect();

    let gcd = gcd2(vec![dimensional_repeat_counts[0], dimensional_repeat_counts[1]]) *
        gcd2(vec![dimensional_repeat_counts[1], dimensional_repeat_counts[2]]) *
        gcd2(vec![dimensional_repeat_counts[0], dimensional_repeat_counts[2]]) /
        gcd2(dimensional_repeat_counts.clone());

    let final_result = dimensional_repeat_counts[0] *
        dimensional_repeat_counts[1] *
        dimensional_repeat_counts[2] /
        gcd;

    return Answer { question: input.question, result: final_result };
}

fn gcd2(mut nums: Vec<usize>) -> usize {
    nums.sort();

    let minimum_value = nums.first().unwrap();

    let mut remainders: Vec<usize> = nums[1..].iter()
        .map(|x| x % minimum_value)
        .filter(|x| x > &0)
        .collect();

    if remainders.len() == 0 {
        *minimum_value
    } else {
        remainders.push(*minimum_value);
        gcd2(remainders)
    }
}
