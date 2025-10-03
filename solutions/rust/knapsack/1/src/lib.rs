use itertools::Itertools;

#[derive(Debug)]
pub struct Item {
    pub weight: u32,
    pub value: u32,
}

pub fn maximum_value(max_weight: u32, items: &[Item]) -> u32 {
    (1..=items.len())
        .flat_map(|k| items.iter().combinations(k))
        .filter(|item_list| item_list.iter().map(|item| item.weight).sum::<u32>() <= max_weight)
        .map(|item_list| item_list.iter().map(|item| item.value).sum::<u32>())
        .sorted_unstable()
        .last()
        .unwrap_or(0)
}
