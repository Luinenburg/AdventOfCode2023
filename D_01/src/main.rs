use std::fs;

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
        match num.parse() {
            Ok(n) => nums.push(n),
            Err(e) => println!("oh shit"),
        }
    }
    let mut i = 0;
    while let Some(y) = nums.pop()
    {
        i += y;
    }
    println!("{i}");
}
