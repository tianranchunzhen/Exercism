use std::collections::HashMap;

use itertools::Itertools;

pub fn tally(match_results: &str) -> String {
    if match_results.trim().is_empty() {
        return "Team                           | MP |  W |  D |  L |  P".to_string();
    }

    let mut record: HashMap<&str, Vec<u32>> = HashMap::new();

    match_results.split("\n").for_each(|line| {
        let line_vec: Vec<&str> = line.split(";").collect();

        let team1 = line_vec[0];
        let team2 = line_vec[1];
        record.entry(team1).or_insert(Vec::from([0, 0, 0, 0, 0]));
        record.entry(team2).or_insert(Vec::from([0, 0, 0, 0, 0]));

        match line_vec[2] {
            "win" => {
                record.entry(team1).and_modify(|v| {
                    v[0] += 1;
                    v[1] += 1;
                    v[4] += 3;
                });
                record.entry(team2).and_modify(|v| {
                    v[0] += 1;
                    v[3] += 1;
                });
            }
            "loss" => {
                record.entry(team2).and_modify(|v| {
                    v[0] += 1;
                    v[1] += 1;
                    v[4] += 3;
                });
                record.entry(team1).and_modify(|v| {
                    v[0] += 1;
                    v[3] += 1;
                });
            }
            "draw" => {
                record.entry(team1).and_modify(|v| {
                    v[0] += 1;
                    v[2] += 1;
                    v[4] += 1;
                });
                record.entry(team2).and_modify(|v| {
                    v[0] += 1;
                    v[2] += 1;
                    v[4] += 1;
                });
            }
            _ => (),
        }
    });

    let mut output = record
        .into_iter()
        .sorted_by_key(|(k, v)| (std::cmp::Reverse(*v.last().unwrap()), *k))
        .map(|(k, v)| {
            format!(
                "{:30} | {:>2} | {:>2} | {:>2} | {:>2} | {:>2}",
                k, v[0], v[1], v[2], v[3], v[4]
            )
        })
        .collect::<Vec<String>>();
    output.insert(
        0,
        "Team                           | MP |  W |  D |  L |  P".to_string(),
    );
    output.join("\n")
}
