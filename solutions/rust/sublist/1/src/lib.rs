#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    if first_list == second_list {
        return Comparison::Equal;
    }

    let mut if_sub = false;
    let mut if_super = false;

    if first_list.is_empty() && !second_list.is_empty() {
        return Comparison::Sublist;
    } else if !first_list.is_empty() && second_list.is_empty() {
        return Comparison::Superlist;
    } else {
        if_sub = second_list
            .windows(first_list.len())
            .any(|sub_list| sub_list == first_list);
        if_super = first_list
            .windows(second_list.len())
            .any(|sub_list| sub_list == second_list);
    }

    match (if_sub, if_super) {
        (false, true) => Comparison::Superlist,
        (true, false) => Comparison::Sublist,
        (false, false) => Comparison::Unequal,
        (true, true) => Comparison::Equal,
    }
}
