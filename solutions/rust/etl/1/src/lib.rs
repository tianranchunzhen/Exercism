use std::{char, collections::BTreeMap};

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut result: BTreeMap<char, i32> = BTreeMap::new();
    for (point, letters) in h {
        for letter in letters {
            for lower_letter in letter.to_lowercase() {
                result.insert(lower_letter, *point);
            }
        }
    }
    result
}
