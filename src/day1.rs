use anyhow::Result;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

pub fn process(path: PathBuf) -> Result<u128> {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let max_value = contents
        .split("\n\n")
        .collect::<Vec<_>>()
        .into_iter()
        .fold(vec![], |mut result, v| {
            let temp_vector = v
                .split('\n')
                .map(|v| v.parse::<u128>().unwrap_or(0))
                .collect::<Vec<_>>();
            result.push(temp_vector);
            result
        })
        .into_iter()
        .fold(vec![], |mut result, v| {
            result.push(v.into_iter().sum::<u128>());
            result
        })
        .into_iter()
        .max()
        .unwrap_or(0);

    Ok(max_value)
}

#[test]
fn test_process() {
    use std::str::FromStr;
    let path =
        PathBuf::from_str("/Users/davirain/adventofcode-rs/tests/fixtures/day1.txt").unwrap();
    let result = process(path);
    println!("result = {:?}", result);
    assert_eq!(72602, result.unwrap());
}
