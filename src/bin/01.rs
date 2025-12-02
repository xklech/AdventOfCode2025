advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let mut position = 50;
    let mut counter = 0;
    for line in input.lines() {
        let rotation = line[1..].parse::<i32>().unwrap() % 100;
        position  += if line.starts_with('L') { -1 } else { 1 } * rotation;
        if position < 0 {
            position = 100 - position.abs();
        } else if position > 99 {
            position = position % 100;
        }
        if position == 0 {
            counter += 1;
        }
        println!("Current position: {}", position);
    }
    Some(counter)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut position = 50;
    let mut counter = 0;
    for line in input.lines() {
        let number = line[1..].parse::<i32>().unwrap();
        if number >= 100 {
            println!("Large rotation detected: {}", number);
        }
        let num_rotations = number / 100;
        counter += num_rotations as u64;
        let rotation_reminder = number % 100;
        position  += if line.starts_with('L') { -1 } else { 1 } * rotation_reminder;
        if position < 0 {
            counter += if num_rotations > 0 { 0 } else { 1 };
            position = 100 - position.abs();
        } else if position > 99 {
            counter += if num_rotations > 0 { 0 } else { 1 };
            position = position % 100;
        }
        println!("Current position: {}", position);
    }
    Some(counter)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
