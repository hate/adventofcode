// each rucksack has 2 compartments (same length)
// each item type can only go in 1 compartment
// find sum of values for same chars in both compartments

fn main() {
    let input = include_str!("../input.txt").lines();

    let first_count = input
        .clone()
        .map(|l| {
            let mid = l.len() / 2;
            let mut found = [0; 53];

            for (i, c) in l.chars().enumerate() {
                let priority = get_priority(c);

                if i < mid {
                    found[priority] = 1;
                } else {
                    if found[priority] == 1 {
                        return priority;
                    }
                }
            }

            0
        })
        .sum::<usize>();

    let second_count = input
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|chunks| {
            let mut found = [0; 53];

            for (i, sack) in chunks.iter().enumerate() {
                for c in sack.chars() {
                    let priority = get_priority(c);

                    if found[priority] == i {
                        if i == 2 {
                            return priority;
                        } else {
                            found[priority] = i + 1;
                        }
                    }
                }
            }
            0
        })
        .sum::<usize>();

    println!("{}", first_count);
    println!("{}", second_count);
}

fn get_priority(c: char) -> usize {
    match c {
        'a'..='z' => c as usize - 96,
        'A'..='Z' => c as usize - 38,
        _ => unreachable!(),
    }
}
