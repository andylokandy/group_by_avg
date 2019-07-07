use std::fs::File;
use std::io::{self, Read};

use group_by_avg::group_by_avg;

fn main() -> Result<(), io::Error> {
    let mut buf = String::new();
    let mut file = File::open("examples/data.txt")?;
    file.read_to_string(&mut buf)?;

    let paris: Vec<(i64, i64)> = buf
        .lines()
        .map(|line| {
            let mut iter = line.split_whitespace();
            (
                iter.next().unwrap().parse().unwrap(),
                iter.next().unwrap().parse().unwrap(),
            )
        }).collect();

    let results = group_by_avg(&paris);

    println!("Results of Group By Avg:");
    for result in results {
        println!("{} -> {}", result.0, result.1);
    }

    Ok(())
}
