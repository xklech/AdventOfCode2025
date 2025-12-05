advent_of_code::solution!(5);

#[derive(Debug)]
struct Range {
    start: u64,
    end: u64,
}

impl Range {
    fn contains(&self, item:u64)->bool {
        item >= self.start && item <= self.end
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let (ranges, itens) = parse_data(input);
    println!("Ranges {:?}", ranges);
    println!("Items {:?}", itens);

    let sum = itens.iter().map(|item| {
        for range in &ranges {
            if range.contains(*item) {
                return 1;
            }
        }
        0
    }).sum();

    Some(sum)
}



fn parse_data(input: &str) -> (Vec<Range>, Vec<u64>) {
     let mut isRange = true;
    let mut ranges: Vec<Range> = Vec::new();
    let mut items: Vec<u64> = Vec::new();
    for line in input.lines()  {
        if (line.is_empty()) {
            isRange = false;
            continue;
        }
        if (isRange) {
            let split: Vec<&str> = line.split('-').collect();
            ranges.push(Range {
                start: split[0].parse::<u64>().unwrap(),
                end: split[1].parse::<u64>().unwrap()
            });
        } else {
            items.push(line.parse::<u64>().unwrap());
        }
    }
    (ranges, items)
}



pub fn part_two(input: &str) -> Option<u64> {
    None
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
        assert_eq!(result, None);
    }
}
