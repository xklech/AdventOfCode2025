advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    let mut joltage = 0;
    for bank in input.lines() {
        println!("Bank: {}", bank);
        let first_battery_position = find_first_largest(bank, bank.len()-2);
        println!("First battery position: {}", first_battery_position);
        let remainder = &bank[first_battery_position + 1..];
        println!("Remainder: {}", remainder);
        let second_battery_local = find_first_largest(remainder, remainder.len()-1);
        let second_battery_position = first_battery_position + second_battery_local + 1;
        println!("Second battery position: {}", second_battery_position);

        let first_char = bank.chars().nth(first_battery_position).unwrap();
        let second_char = bank.chars().nth(second_battery_position).unwrap();
        let max_joltage = first_char.to_string() + &second_char.to_string();
        println!("Max bank joltage {}", max_joltage);
        joltage += max_joltage.parse::<u64>().unwrap();

    }
    Some(joltage)
}

fn find_first_largest(bank: &str, max_position: usize) -> usize {
    for i in (1..=9).rev() {
        let find_first_occurance = bank.find(char::from_digit(i, 10).unwrap());
        if find_first_occurance.is_some() {
            let first = find_first_occurance.unwrap();
            if first <= max_position {
                return first;
            }
        }
    }
    0
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut joltage = 0;
    let battery_count = 12;
    for bank in input.lines() {
        let mut max_joltage = String::new();
        let mut battery_position = 0;
        println!("Bank: {}", bank);
        for i in 0..battery_count {
            let remainder = &bank[battery_position..];
            let max_position = remainder.len() + i - battery_count;
            let battery_sub_position = find_first_largest(remainder, max_position);
            max_joltage.push(bank.chars().nth(battery_position + battery_sub_position).unwrap());
            battery_position += battery_sub_position + 1;
        }
        println!("Max bank joltage {}", max_joltage);
        joltage += max_joltage.parse::<u64>().unwrap();

    }
    Some(joltage)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
