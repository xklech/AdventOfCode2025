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

    fn item_count(&self)-> u64 {
        self.end - self.start + 1
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
     let mut is_range = true;
    let mut ranges: Vec<Range> = Vec::new();
    let mut items: Vec<u64> = Vec::new();
    for line in input.lines()  {
        if line.is_empty() {
            is_range = false;
            continue;
        }
        if is_range {
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
    // range by range, add or expand exisitng ranges
    let (mut ranges, _itens) = parse_data(input);
    while merge_ranges(&mut ranges) {
        println!("Merging Ranges Len {:?}", ranges.len());
    }
    println!("Final Ranges {:?}", ranges);
    Some(ranges.iter().map(|r| r.item_count()).sum())
}

fn merge_ranges(ranges: &mut Vec<Range>) -> bool {
    let mut final_ranges: Vec<Range> = vec![];
    for range in ranges.iter() {
        let mut added = false;
        for final_range in &mut final_ranges {
            if (range.start <= final_range.end + 1) && (range.end >= final_range.start - 1) {
                final_range.start = final_range.start.min(range.start);
                final_range.end = final_range.end.max(range.end);
                added = true;
                break;
            }
        }
        if !added {
            final_ranges.push(Range {
                start: range.start,
                end: range.end,
            })
        }
    }
    let changed = final_ranges.len() != ranges.len();
    *ranges = final_ranges;
    return changed;
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
        assert_eq!(result, Some(14));
    }
}
