use anyhow::Result;

pub fn fetch_puzzle_input(puzzle_number: u8) -> Result<String> {
    let aoc_token = std::env::var("AOC_2024_TOKEN")?;

    let client = reqwest::blocking::Client::new();

    let now = std::time::Instant::now();
    let resp = client
        .get(format!(
            "https://adventofcode.com/2024/day/{}/input",
            puzzle_number
        ))
        .header("Cookie", aoc_token)
        .send()?;
    println!("Request Time = {}ms", now.elapsed().as_millis());

    Ok(resp.text()?)
}
