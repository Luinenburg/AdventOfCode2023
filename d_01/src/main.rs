fn find_numbers_in_string(line_with_nums: &str) -> Vec<(usize, usize)>
{
    let numbers_as_string = vec!["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let numbers_as_numbers = vec!["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
    let mut indexes_of_numbers: Vec<(usize, usize)> = Vec::new();
    for (index, numbers) in numbers_as_string.iter().enumerate()
    {
        match line_with_nums.find(numbers)
        {
            Some(n) => indexes_of_numbers.push((n, index)),
            None => continue,
        }
        match line_with_nums.rfind(numbers)
        {
            Some(n) => indexes_of_numbers.push((n, index)),
            None => continue,
        }
    }
    for (index, numbers) in numbers_as_numbers.iter().enumerate()
    {
        match line_with_nums.find(numbers)
        {
            Some(n) => indexes_of_numbers.push((n, index)),
            None => continue,
        }
        match line_with_nums.rfind(numbers)
        {
            Some(n) => indexes_of_numbers.push((n, index)),
            None => continue,
        }
    }
    indexes_of_numbers
}

fn mins_and_maxes(set: &[(usize, usize)]) -> usize
{
    if set.len() == 1 { return set[0].1 + set[0].1*10; }
    let mut min_num = 0;
    let mut min_index = 100000;
    let mut max_num = 0;
    let mut max_index = 0;
    for pair in set
    {
        if pair.0 < min_index
        {
            min_index = pair.0;
            min_num = pair.1;
        }
        if pair.0 >= max_index
        {
            max_index = pair.0;
            max_num = pair.1;
        }
    }
    min_num*10 + max_num
}

fn main()
{
    let file_contents: Vec<&str> = include_str!("input.txt")
        .trim_end()
        .split("\n")
        .collect();
    let mut nums: Vec<i32> = Vec::new();
    
    for y in &file_contents
    {
        let mut num: String = String::new();
        for c in y.chars()
        { if c.is_numeric() { num.push(c); break; } }
        for c in y.chars().rev()
        { if c.is_numeric() { num.push(c); break; } }
        if num == "" { continue; }
        match num.parse() {
            Ok(n) => nums.push(n),
            Err(_e) => println!("{num}"),
        }
    }

    let mut sum = 0;
    for y in &file_contents
    {
        sum += mins_and_maxes(&find_numbers_in_string(y));
        println!("{} {}", mins_and_maxes(&find_numbers_in_string(y)), y)
    }
    let mut i = 0;
    while let Some(y) = nums.pop()
    {
        i += y;
    }
    println!("{i}");
    println!("{}", sum);
    println!("{:#?}", find_numbers_in_string(file_contents[0]))
}
