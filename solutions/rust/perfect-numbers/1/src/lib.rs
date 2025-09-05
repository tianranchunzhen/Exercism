use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

fn get_factors(num: u64) -> Vec<u64> {
    let mut res = vec![1];
    for i in 2..num.isqrt() + 1 {
        if num % i == 0 {
            if i == num / i {
                res.push(i);
            } else {
                res.push(i);
                res.push(num / i);
            }
        }
    }
    res
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {
        None
    } else {
        let sum_of_factors: u64 = get_factors(num).iter().sum();
        if sum_of_factors == 1 {
            Some(Classification::Deficient)
        } else {
            match num.cmp(&sum_of_factors) {
                Ordering::Less => Some(Classification::Abundant),
                Ordering::Equal => Some(Classification::Perfect),
                Ordering::Greater => Some(Classification::Deficient),
            }
        }
    }
}
