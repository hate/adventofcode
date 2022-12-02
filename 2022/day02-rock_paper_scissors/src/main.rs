// first column/line is what opponent plays
// A/X for Rock, B/Y for Paper, and C/Z for Scissors
// Shape you selected (1 for Rock, 2 for Paper, and 3 for Scissors)
// Outcome of the round (0 if you lost, 3 if the round was a draw, and 6 if you won)
// Part 2: X means lose, Y means draw, Z means win

fn main() {
    let (first, second) = include_str!("../input.txt")
        .lines()
        .map(|l| l.split_once(" ").unwrap())
        .fold(
            (0, 0),
            |(mut first_score, mut second_score), (opponent, player)| {
                first_score += get_round_score((opponent, player));
                second_score += get_round_score((opponent, &get_desired_hand((opponent, player))));

                (first_score, second_score)
            },
        );

    // Part 1
    println!("{}", first);

    // Part 2
    println!("{}", second);
}

fn get_round_score((opponent, player): (&str, &str)) -> u32 {
    let shape_score = match player {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => unreachable!(),
    };

    let outcome_score = match (opponent, player) {
        ("A", "Y") | ("B", "Z") | ("C", "X") => 6,
        ("A", "Z") | ("B", "X") | ("C", "Y") => 0,
        _ => 3,
    };

    shape_score + outcome_score
}

fn get_desired_hand((opponent, outcome): (&str, &str)) -> String {
    if outcome == "Z" {
        // win
        match opponent {
            "A" => "Y".to_owned(),
            "B" => "Z".to_owned(),
            "C" => "X".to_owned(),
            _ => unreachable!(),
        }
    } else if outcome == "X" {
        // lose
        match opponent {
            "A" => "Z".to_owned(),
            "B" => "X".to_owned(),
            "C" => "Y".to_owned(),
            _ => unreachable!(),
        }
    } else {
        // draw
        match opponent {
            "A" => "X".to_owned(),
            "B" => "Y".to_owned(),
            "C" => "Z".to_owned(),
            _ => unreachable!(),
        }
    }
}
