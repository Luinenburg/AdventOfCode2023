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
    let powers: Vec<usize> = data_input
        .iter()
        .map(|data| data.1.iter().filter(|x| data.0.contains(x)).count())
        .collect();
    let mut sum = 0;
    for i in powers {
        if i > 0 {
            sum += 1 << (i - 1)
        };
    }

    println!("{}", sum);
}
