use std::env;
use std::fs::read_to_string;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = Path::new(&args[1]);
    let mut lists = vec![vec![], vec![]];
    dbg!(input_file);
    if input_file.exists() {
        let file = read_to_string(input_file);
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
        let answer: i32 = lists[0]
            .clone()
            .into_iter()
            .zip(lists[1].clone())
            .fold(0, |acc, (n1, n2)| acc + (n1 - n2).abs());
        println!("{}", answer);
    }
}
