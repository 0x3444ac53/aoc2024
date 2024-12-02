use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;
use std::path::Path;

fn get_lists(file: &Path) -> Vec<Vec<i32>> {
    let mut lists = vec![vec![], vec![]];
    if file.exists() {
        let file = read_to_string(file);
        file.unwrap().lines().for_each({
            |line| {
                let nums: Vec<&str> = line.split_whitespace().collect();
                let n1: i32 = String::from(nums[0]).parse().unwrap();
                let n2: i32 = String::from(nums[1]).parse().unwrap();
                lists[0].push(n1);
                lists[1].push(n2);
            }
        });
        lists[0].sort();
        lists[1].sort();
    }
    lists
}

fn get_occurances(list: Vec<i32>) -> HashMap<i32, i32> {
    list.into_iter().fold(HashMap::new(), |mut acc, n| {
        *acc.entry(n).or_insert(0) += 1;
        acc
    })
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = Path::new(&args[1]);
    let lists = get_lists(input_file);
    let answer1: i32 = lists[0]
        .clone()
        .into_iter()
        .zip(lists[1].clone())
        .fold(0, |acc, (n1, n2)| acc + (n1 - n2).abs());
    println!("Part1: {}", answer1);
    let occs = get_occurances(lists[1].clone());
    let answer2 = lists[0]
        .clone()
        .into_iter()
        .map(|n| n * occs.get(&n).unwrap_or(&0))
        .reduce(|acc, n| acc + n)
        .unwrap();
    println!("Part2: {}", answer2);
}
