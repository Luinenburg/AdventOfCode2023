use std::fs;

fn find_number_in_string(line_with_nums: &str) -> i32
{
    let nums_as_string = vec!["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let mut possible_num_indexes: Vec<usize> = Vec::new();
    for idk_a_name in nums_as_string
    {
        match line_with_nums.find(idk_a_name)
        {
            Some(n) => possible_num_indexes.push(n),
            None => continue,
        }
    }
    for n in possible_num_indexes {
        println!("{n}");
    }
    return 0;    
}

fn main()
{
    let binding = fs::read_to_string("example.txt")
        .expect("Should have been able to read file");
    let mut file_contents: Vec<_> = binding.split("\n").collect();
    let mut nums: Vec<i32> = Vec::new();
    
    while let Some(y) = file_contents.pop()
    {
        find_number_in_string(y);
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

    let mut i = 0;
    while let Some(y) = nums.pop()
    {
        i += y;
    }
    println!("{i}");
}
