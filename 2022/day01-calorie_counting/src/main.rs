// reindeer need magical energy to delive presents
// reindeer eat stars
// need to retrieve 50 stars by dec 25
// 2 puzzles are available each day

// elves take inventory of the Calories each one is carrying
// elves want to figure out the one with the highest count
// inventory found in input.txt
// elves want to figure out the total of the top 3

fn main() {
    let mut elves = include_str!("../input.txt")
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .map(|calories| calories.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect::<Vec<u32>>();

    // Part 1
    println!("{}", elves.iter().max().unwrap());

    elves.sort_unstable();

    // Part 2
    println!("{}", elves.iter().rev().take(3).sum::<u32>());
}
