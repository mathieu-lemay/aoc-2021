use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::{Add, Mul, Sub};
use std::str::FromStr;

pub fn get_input(filename: &str) -> Vec<String> {
    let file = match File::open(format!("input/{}", filename)) {
        Ok(file) => file,
        Err(error) => panic!("Unable to open file {}: {}", filename, error),
    };

    let reader = BufReader::new(file);

    reader.lines().map(|l| l.unwrap()).collect()
}

pub fn get_input_as_int<T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Ord + FromStr>(
    filename: &str,
) -> Vec<T>
where
    <T as FromStr>::Err: Debug,
{
    get_input(filename)
        .iter()
        .map(|i| i.parse().unwrap())
        .collect()
}
