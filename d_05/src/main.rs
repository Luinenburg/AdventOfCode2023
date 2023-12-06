use itertools::Itertools;

fn generate_path_of_seed(seed: usize, maps: &[Vec<(usize, usize, usize)>]) -> Vec<usize>
{
    let mut current_step: usize = seed;
    let mut path = vec![current_step];
    for map in maps {
        for (destination, source, offset) in map {
            if *source > current_step || *source + *offset < current_step {
                continue;
            }
            current_step = *destination + current_step - *source;
            break;
        }
        path.push(current_step);
    }
    return path;
}

fn follow_seed_without_gen(seed: usize, maps: &[Vec<(usize, usize, usize)>]) -> usize
{
    let mut current_step: usize = seed;
    for map in maps {
        for (destination, source, offset) in map {
            if *source > current_step || *source + *offset < current_step {
                continue;
            }
            current_step = *destination + current_step - *source;
            break;
        }
    }
    return current_step;
}

fn generate_path_of_location(location: usize, maps: &[Vec<(usize, usize, usize)>]) -> Vec<usize>
{
    let mut current_step: usize = location;
    let mut path: Vec<usize> = vec![current_step];
    for map in maps.into_iter().rev().collect::<Vec<&Vec<(usize, usize, usize)>>>() {
        for (destination, source, offset) in map {
            if *destination > current_step || *destination + *offset < current_step {
                continue;
            }
            current_step = *source + current_step - *destination;
            break;
        }
        path.push(current_step)
    }
    path
}

fn follow_path_of_locations(location_max: usize, seed_ranges: &[(usize, usize)], maps: &[Vec<(usize, usize, usize)>]) -> usize
{
    for location in 0..=location_max {
        let seed = *generate_path_of_location(location, maps).last().unwrap();
        for (seed_min, seed_max) in seed_ranges {
            if seed >= *seed_min && seed <= *seed_max {
                return location;
            }
        }
    }
    0
}

fn follow_path_of_seed_ranges(seed_ranges: &[(usize, usize)], maps: &[Vec<(usize, usize, usize)>]) -> usize
{
    let num = seed_ranges.iter().map(|seed_range| {
        (seed_range.0..=seed_range.1).map(|seed| {
            follow_seed_without_gen(seed, maps)
        }).min().unwrap()
    }).min().unwrap();
    println!("{:#?}", num);
    num
}

fn main() {
    let data_input: Vec<Vec<&str>> = include_str!("example.txt")
        .split("\n\n")
        .map(|x|
            x.lines()
                .collect::<Vec<_>>())
        .collect();

    let seeds: Vec<usize> = data_input[0][0].strip_prefix("seeds: ")
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<usize>()
            .unwrap())
        .collect();

    let seed_ranges: Vec<(usize, usize)> = {
        let mut ls: Vec<(usize, usize)> = Vec::new();
        for i in (0..seeds.len()).step_by(2) {
            ls.push((seeds[i], seeds[i] + seeds[i + 1]));
        }
        ls.sort();
        ls
    };

    let maps: &Vec<Vec<(usize, usize, usize)>> = &data_input[1..]
        .iter()
        .map(|section: &Vec<&str>| {
            let mut val = section[1..]
                .iter()
                .map(|line: &&str|
                    line.split_whitespace()
                        .flat_map(str::parse)
                        .collect_tuple().unwrap())
                .collect::<Vec<_>>();
            val.sort();
            val
        })
        .collect();

    let location_max: usize = maps.last()
        .unwrap()
        .last()
        .unwrap()
        .0 + maps
        .last()
        .unwrap()
        .last()
        .unwrap()
        .2;

    let minimum: usize = seeds
        .iter()
        .map(|seed| {
            let s = generate_path_of_seed(*seed, maps);
            *s.last()
                .unwrap()
        })
        .min()
        .unwrap();

    println!("{:#?}", minimum);
    println!("{:#?}", follow_path_of_locations(location_max, &seed_ranges, maps));
    follow_path_of_seed_ranges(&seed_ranges, maps);
}
