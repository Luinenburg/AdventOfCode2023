fn wins_per_game(data_input: &[(Vec<i32>, Vec<i32>)]) -> Vec<usize>
{
    data_input
        .iter()
        .map(|data| data.1.iter().filter(|x| data.0.contains(x)).count())
        .collect()
}

fn part_two(data_input: &[(Vec<i32>, Vec<i32>)]) -> i32 {
    unimplemented!()
}

fn part_one(data_input: &[(Vec<i32>, Vec<i32>)]) -> i32 {
    let powers = wins_per_game(data_input);
    let mut sum = 0;
    for i in powers {
        if i > 0 {
            sum += 1 << (i - 1)
        };
    }
    sum
}

fn main() {
    let data_input: Vec<(Vec<i32>, Vec<i32>)> = include_str!("input.txt")
        .trim_end()
        .lines()
        .map(|line| {
            let split_line = line.split_once(": ").unwrap().1.split_once(" | ").unwrap();
            (
                split_line.0.split(' ').flat_map(str::parse).collect(),
                split_line.1.split(' ').flat_map(str::parse).collect(),
            )
        })
        .collect();



    println!("{}", part_one(&data_input));
    println!("{}", part_two(&data_input));
}
