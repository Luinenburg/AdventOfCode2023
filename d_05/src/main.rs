use itertools::Itertools;

<<<<<<< Updated upstream
fn generate_path_of_seed(seed: usize, maps: &[Vec<(usize, usize, usize)>]) -> Vec<usize> {
=======
fn navigate_map(map: &Vec<(usize, usize, usize)>, step: usize) -> usize {
    let mut current_step = step;
    for (destination, source, offset) in map {
        if *source > current_step || *source + *offset < current_step {
            continue;
        }
        current_step = *destination + current_step - *source;
        break;
    }
    current_step
}

fn generate_path_of_seed(seed: usize, maps: &[Vec<(usize, usize, usize)>]) -> Vec<usize>
{
>>>>>>> Stashed changes
    let mut current_step: usize = seed;
    let mut path = vec![current_step];
    for map in maps {
        current_step = navigate_map(map, current_step);
        path.push(current_step);
    }
    path
}

fn follow_seed_without_gen(seed: usize, maps: &[Vec<(usize, usize, usize)>]) -> usize {
    let mut current_step: usize = seed;
    for map in maps {
        current_step = navigate_map(map, current_step);
    }
    current_step
}

fn generate_path_of_location(location: usize, maps: &[Vec<(usize, usize, usize)>]) -> Vec<usize> {
    let mut current_step: usize = location;
    let mut path: Vec<usize> = vec![current_step];
<<<<<<< Updated upstream
    for map in maps
        .into_iter()
        .rev()
        .collect::<Vec<&Vec<(usize, usize, usize)>>>()
    {
        for (destination, source, offset) in map {
            if *destination > current_step || *destination + *offset < current_step {
                continue;
            }
            current_step = *source + current_step - *destination;
            break;
        }
=======
    for map in maps.into_iter().rev().collect::<Vec<&Vec<(usize, usize, usize)>>>() {
        current_step = navigate_map(map, current_step);
>>>>>>> Stashed changes
        path.push(current_step)
    }
    path
}

fn follow_path_of_locations(
    location: usize,
    seed_ranges: &[(usize, usize)],
    maps: &[Vec<(usize, usize, usize)>],
) -> usize {
    let seed = *generate_path_of_location(location, maps).last().unwrap();
    for (seed_min, seed_max) in seed_ranges {
        if seed >= *seed_min && seed <= *seed_max {
            return location;
        }
    }
    0
}

fn follow_path_of_seed_ranges(
    seed_ranges: &[(usize, usize)],
    maps: &[Vec<(usize, usize, usize)>],
) -> usize {
    let num = seed_ranges
        .iter()
        .map(|seed_range| {
            (seed_range.0..=seed_range.1)
                .map(|seed| follow_seed_without_gen(seed, maps))
                .min()
                .unwrap()
        })
        .min()
        .unwrap();
    num
}

fn main() {
    let data_input: Vec<Vec<&str>> = include_str!("input.txt")
        .split("\n\n")
        .map(|x| x.lines().collect::<Vec<_>>())
        .collect();

    let seeds: Vec<usize> = data_input[0][0]
        .strip_prefix("seeds: ")
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
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
                .map(|line: &&str| {
                    line.split_whitespace()
                        .flat_map(str::parse)
                        .collect_tuple()
                        .unwrap()
                })
                .sorted()
                .collect::<Vec<_>>();
            val.sort();
            val
        })
        .collect();

    let locations: Vec<(usize, usize)> = {
        maps.last()
            .unwrap()
            .into_iter()
            .map(|humidity_to_location| {
                (
                    humidity_to_location.0,
                    humidity_to_location.0 + humidity_to_location.2,
                )
            })
            .collect()
    };

    let location_max: usize =
        maps.last().unwrap().last().unwrap().0 + maps.last().unwrap().last().unwrap().2;

    let minimum: usize = seeds
        .iter()
        .map(|seed| {
            let s = generate_path_of_seed(*seed, maps);
            *s.last().unwrap()
        })
        .min()
        .unwrap();

    println!("{:#?}", minimum);
    println!("{:#?}", locations);
    for location in locations[0].0..locations[0].1 {
        let seed = follow_path_of_locations(location, &seed_ranges, maps);
        if seed == 0 {
            continue;
        }
        println!("{:#?}", seed);
        if seed != 0 {
            break;
        }
    }
    println!("{:#?}", follow_path_of_seed_ranges(&seed_ranges, maps));
}
