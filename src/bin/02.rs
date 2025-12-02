use core::str;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let ranges: Vec<&str> = input.split(',').collect();
    let mut sum: u64 = 0;
    for range in ranges {
        let splitted: Vec<&str> = range.split('-').collect();
        let start: u64 = splitted[0].parse().unwrap();
        let end: u64 = splitted[1].parse().unwrap();
        println!("Start: {}, End: {}", start, end);

        for num in start..=end {
            let str_num = num.to_string();
            let halfs = str_num.split_at(str_num.len() / 2);
            if halfs.0 == halfs.1  {
                println!("Found same: {}", num);
                sum += num;
            }
        }
    }
    return Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let ranges: Vec<&str> = input.split(',').collect();
    let mut sum: u64 = 0;
    for range in ranges {
        let splitted: Vec<&str> = range.split('-').collect();
        let start: u64 = splitted[0].parse().unwrap();
        let end: u64 = splitted[1].parse().unwrap();
        println!("Start: {}, End: {}", start, end);

        for num  in start..=end {
            let num_str = num.to_string();
            for i in 1..(num_str.len()/2)+1 {
                let part = &num_str[0..i];
                let repeated = part.repeat(num_str.len()/i);
                if repeated == num_str {
                    println!("Found repeated: {}", num);
                    sum += num;
                    break;
                }
            }
        }
    }
    return Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
