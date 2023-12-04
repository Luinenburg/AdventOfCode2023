use std::collections::HashSet;

fn find_complete_number(data: &[&str], position: (usize, usize)) -> usize {
    let mut center: isize = -1;
    if position.1==data[position.0].len()-1 { center = -2; }
    if position.1==0 { center = 0; }
    if !data[position.0].chars().nth(position.1+1).unwrap().is_numeric() { center = -2; }
    if !data[position.0].chars().nth(position.1-1).unwrap().is_numeric() { center = 0; }

    let mut num_as_str = String::new();
    for i in center..=center+2
    {
        if data[position.0].chars().nth(position.1.wrapping_add_signed(i)).unwrap().is_numeric() {
            num_as_str.push(data[position.0].chars().nth(position.1.wrapping_add_signed(i)).unwrap());
        }
    }
    num_as_str.parse().unwrap()
}

fn find_nums_around_symbol(symbol: char, data: &[&str], position: (isize, isize)) -> (char, HashSet<usize>) {
    let mut nums_around_symbol: HashSet<usize> = HashSet::new();
    for mut row_mod in -1..=1 {
        for mut column_mod in -1..=1 {
            if position.0 + row_mod < 0 || position.0 + row_mod >= data.len() as isize {row_mod = 0;}
            let row = (position.0+row_mod) as usize;
            if position.1 + column_mod < 0 || position.1 + column_mod >= data[row].len() as isize {column_mod = 0;}
            let column = (position.1+column_mod) as usize;

            if data[row].chars().nth(column).unwrap().is_numeric() {
                nums_around_symbol.insert(find_complete_number(data, (row, column)));
            }
        }
    }
    (symbol, nums_around_symbol)
}

fn main() {
    let data_input: Vec<&str> = include_str!("input.txt")
        .trim_end()
        .split('\n')
        .collect();

    let mut symbols_with_nums: Vec<(char, HashSet<usize>)> = vec![];

    for (row, line) in data_input.iter().enumerate() {
        let row = row as isize;
        for (column, artifact) in line.chars().enumerate() {
            let column = column as isize;
            if !(artifact.is_numeric() || artifact == '.') {
                let something = find_nums_around_symbol(artifact, &data_input, (row, column));
                symbols_with_nums.push(something);
            }
        }
    }

    let sum: usize = symbols_with_nums.iter().flat_map(|(_, set)| set).sum();

    println!("{}", sum);

    let mut product = 0usize;

    for (artifact, nums) in symbols_with_nums {
        if artifact == '*' && nums.len()==2 {
            product += nums.iter().product::<usize>();
        }
    }

    println!("{}", product);
}
