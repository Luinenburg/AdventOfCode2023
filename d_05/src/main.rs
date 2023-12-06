fn follow_path_of_seed(seed: usize, maps: &[Vec<Vec<usize>>]) -> Vec<usize>
{
    // destination, source, offset
    let mut current_step = seed;
    let mut path = vec![current_step];
    for map in maps {
        for correspondence in map {
            if correspondence[1] > current_step || correspondence[1] + correspondence[2] < current_step {
                continue;
            }
            current_step = correspondence[0] + current_step - correspondence[1];
            break;
        }
        path.push(current_step);
    }
    return path;
}

fn follow_path_from_location_help(location: usize, maps: &[Vec<Vec<usize>>]) -> usize
{
    let mut current_step = location;
    for map in maps.into_iter().rev() {
        for correspondence in map {
            if correspondence[0] > current_step || correspondence[0] + correspondence[2] < current_step {
                continue;
            }
            current_step = correspondence[1] + current_step - correspondence[0];
            break;
        }
    }
    current_step
}

fn follow_path_from_location(location_ranges: &[(usize, usize)], seed_ranges: &[(usize, usize)], maps: &[Vec<Vec<usize>>]) -> Vec<usize>
{
    let mut possible_path: Vec<usize> = vec![];
    for location_range in location_ranges {
        for location in location_range.0..=location_range.1 {
            let seed = follow_path_from_location_help(location, maps);

            if seed_ranges.iter().map(|range| (range.0 <= seed) && (seed <= range.1)).collect::<Vec<_>>().contains(&true)
            {
                possible_path.push(seed);
            }
        }
    }
    possible_path
}

fn main() {
    let data_input: Vec<_> = include_str!("example.txt")
        .split("\n\n")
        .map(|x|
            x.lines()
                .collect::<Vec<_>>())
        .collect();

    let seeds: Vec<usize> = data_input[0][0].split_once("seeds: ")
        .unwrap()
        .1
        .split(' ')
        .map(|x| x.parse::<usize>().unwrap()).collect();

    let seed_ranges: Vec<(usize, usize)> = {
        let mut ls: Vec<(usize, usize)> = Vec::new();
        for i in (0..seeds.len()-1).step_by(2) {
            ls.push((seeds[i], seeds[i]+seeds[i+1]));
        }
        ls.sort();
        ls
    };

    let maps: &Vec<_> = &data_input[1..]
        .iter()
        .map(|section| section[1..]
            .iter()
            .map(|line|
                line.split_whitespace()
                .flat_map(str::parse::<usize>)
                .collect::<Vec<_>>())
            .collect::<Vec<_>>())
        .collect();

    let mut location_ranges: Vec<(usize, usize)> = {
        let mut lr: Vec<(usize, usize)> = maps
            .last()
            .unwrap()
            .iter()
            .map(|line| (line[0], line[0] + line[2]))
            .collect();
        lr.sort();
        lr
    };

    let minimum: Vec<_> = seeds.iter().map(|seed| follow_path_of_seed(*seed, maps)).collect();

    println!("{:#?}", minimum
        .iter()
        .flat_map(|x|
            x.last())
        .min()
        .unwrap());
    println!("{:#?}", follow_path_from_location(&location_ranges, &seed_ranges, maps).iter().min());
}
