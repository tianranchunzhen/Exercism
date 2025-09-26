use rayon::ThreadPoolBuilder;
use rayon::prelude::*;
use std::collections::HashMap;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let pool = ThreadPoolBuilder::new()
        .num_threads(worker_count)
        .build()
        .expect("并行线程池构建失败");

    pool.install(|| {
        input
            .join("")
            .par_chars()
            .filter_map(|c| {
                if c.is_alphabetic() {
                    Some(c.to_ascii_lowercase())
                } else {
                    None
                }
            })
            .fold(
                || HashMap::new(),
                |mut map, c| {
                    *map.entry(c).or_insert(0) += 1;
                    map
                },
            )
            .reduce(
                || HashMap::new(),
                |mut map1, map2| {
                    for (char, num) in map2 {
                        *map1.entry(char).or_insert(0) += num;
                    }
                    map1
                },
            )
    })
}
