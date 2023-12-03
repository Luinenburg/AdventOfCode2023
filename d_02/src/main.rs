use log::debug;

fn parse_my_balls(line_of_balls: &str) -> Vec<(String, usize)>
{
    let short_line_of_balls: &str = &line_of_balls[5..];
    let split_short_line_of_balls = short_line_of_balls
        .split_once(": ")
        .unwrap()
        .1
        .split("; ")
        .map(|testicles| testicles.split(", "));
    let mut a_couple_rounds_of_ball_game = vec![];
    for round in split_short_line_of_balls
    {
        for hand_of_balls in round
        {
            let (left_testicle, right_testicle) = hand_of_balls.split_once(' ').unwrap();
            a_couple_rounds_of_ball_game.push((right_testicle.to_string(), left_testicle.parse().unwrap()));
        }
    }
    a_couple_rounds_of_ball_game
}

fn part_one(another_gay_joke: &[Vec<(String, usize)>])
{
    let mut bitch_ass_real_games = 0;
    for (gameid, game) in another_gay_joke.iter().enumerate()
    {
        let mut game_succeeds = true;
        for round in game
        {
            if round.0 == "red" && round.1 > 12 { game_succeeds = false; }
            if round.0 == "green" && round.1 > 13 { game_succeeds = false; }
            if round.0 == "blue" && round.1 > 14 { game_succeeds = false; }
        }
        if game_succeeds { bitch_ass_real_games += gameid + 1; }
    }
    println!("{}", bitch_ass_real_games);
}

fn part_two(another_gay_joke: &[Vec<(String, usize)>])
{
    let mut power = 0;
    for game in another_gay_joke
    {
        let mut max_red = 1;
        let mut max_green = 1;
        let mut max_blue = 1;
        for round in game
        {
            match round.0.as_str()
            {
                "red" if round.1 > max_red => { max_red = round.1; }
                "green" if round.1 > max_green => { max_green = round.1; }
                "blue" if round.1 > max_blue => { max_blue = round.1; }
                _ => {}
            }
        }
        power += max_red * max_green * max_blue;
    }
    println!("{}", power);
}

fn main()
{
    let gay_orgy: Vec<&str> = include_str!("input.txt")
        .trim_end()
        .split('\n')
        .collect();

    let another_gay_joke =
    {
        let mut placeholder = vec![];
        for line in gay_orgy
        {
            placeholder.push(parse_my_balls(line));
        }
        placeholder
    };
    part_one(&another_gay_joke);
    part_two(&another_gay_joke);
}
