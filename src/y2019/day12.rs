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

    fn x(p: &Point3D) -> i32 { p.x }
    fn y(p: &Point3D) -> i32 { p.y }
    fn z(p: &Point3D) -> i32 { p.z }

    let dimensions: [fn(&Point3D) -> i32; 3] = [x, y, z];

    let results: Vec<usize> = dimensions.iter().map(|dimension_function| {
        // TODO: Should this start with 0?
        let mut index: usize = 1;
        let mut points: Vec<Point3D> = initial_points.clone();
        let mut velocities: Vec<Point3D> = vec![Point3D::origin(); points.len()];

        loop {
            update_velocities(&points, velocities.as_mut());
            update_positions(points.as_mut(), &velocities);
            index += 1;

            if initial_points.iter().map(dimension_function).collect::<Vec<i32>>() == points.iter().map(dimension_function).collect::<Vec<i32>>() {
                break
            }
        }
        index
    }).collect();

    println!("{:?}", results);

    println!("{}", gcd(results[0], results[2]));
    println!("{}", gcd(results[1], results[2]));
    println!("{}", gcd(results[1], results[0]));

    let final_result = results[0] * results[1] * results[2];

    for i in &results {
        println!("{} {}", i, 332477126821644.0 / *i as f32);
    }

    for i in &results {
        println!("{} {}", i, 83119281705411.0 / *i as f32);
    }

    return Answer { question: input.question, result: final_result };
}

fn gcd(left: usize, right: usize) -> usize {
    if left > right {
        gcd(right, left)
    } else {
        let remain = right % left;
        if remain == 0 {
            left
        } else {
            gcd(left, remain)
        }
    }
}
