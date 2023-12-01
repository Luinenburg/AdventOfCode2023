use std::fs;

fn findNumberInString(lineWithNums: String) -> i32
{
    let mut possibleNumIndexes: Vec<i32> = Vec::new();
    return 0;    
}

fn main()
{
    let binding = fs::read_to_string("input.txt")
        .expect("Should have been able to read file");
    let mut file_contents: Vec<_> = binding.split("\n").collect();

    let mut nums: Vec<i32> = Vec::new();
    
    while let Some(y) = file_contents.pop()
    {
        let mut num: String = String::new();
        for c in y.chars()
        { if c.is_numeric() { num.push(c); break; } }
        for c in y.chars().rev()
        { if c.is_numeric() { num.push(c); break; } }
        if num == "" { continue; }
        match num.parse() {
            Ok(n) => nums.push(n),
            Err(e) => println!("{num}"),
        }
    }
    let mut i = 0;
    while let Some(y) = nums.pop()
    {
        i += y;
    }
    println!("{i}");
}
