fn test_button_time(button: usize, time: usize, distance: usize) -> bool {
    time - button > distance / button
}

fn run_one_race(race: (usize, usize)) -> usize {
    (1..race.0)
        .filter(|&possible_time| test_button_time(possible_time, race.0, race.1))
        .count()
}

fn run_races(races: &[Vec<usize>]) -> Vec<Vec<usize>> {
    races
        .iter()
        .map(|race| {
            (1..race[0])
                .filter(|possible_time| test_button_time(*possible_time, race[0], race[1]))
                .collect::<Vec<_>>()
        })
        .collect()
}

fn part_one(winning_times: &[Vec<usize>]) -> usize {
    winning_times.iter().map(|x| x.len()).product()
}

fn main() {
    let data_input: Vec<Vec<usize>> = include_str!("example.txt")
        .trim_end()
        .lines()
        .map(|line| {
            line.split_once(':')
                .unwrap()
                .1
                .split_whitespace()
                .flat_map(str::parse::<usize>)
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<_>>();
    let data_inverted: &Vec<Vec<usize>> = &(0..data_input[0].len())
        .map(|i| data_input.iter().map(|c| c[i]).collect())
        .collect::<Vec<_>>();
    let fixed_data_input: Vec<_> = include_str!("input.txt")
        .trim_end()
        .lines()
        .map(|line| {
            line.split_once(':')
                .unwrap()
                .1
                .split_whitespace()
                .collect::<String>()
                .parse::<usize>()
                .unwrap()
        })
        .collect::<Vec<_>>();
    let winning_times = run_races(data_inverted);
    println!("{}", part_one(&winning_times));
    println!("{}", run_one_race((fixed_data_input[0], fixed_data_input[1])));
}
