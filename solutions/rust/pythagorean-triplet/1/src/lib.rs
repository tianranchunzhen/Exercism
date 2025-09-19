use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    let mut triples = HashSet::new();
    let max_m = ((sum as f64).sqrt() as u32) + 1;

    for m in 2..=max_m {
        let start_n = if m % 2 == 0 { 1 } else { 2 };
        for n in (start_n..m).step_by(2) {
            if gcd(m, n) != 1 {
                continue;
            }

            let a = m * m - n * n;
            let b = 2 * m * n;
            let c = m * m + n * n;
            let perimeter = a + b + c;

            if perimeter == 0 || perimeter > sum {
                continue;
            }

            if sum % perimeter == 0 {
                let k = sum / perimeter;
                let triple = [a * k, b * k, c * k];
                let mut t = [triple[0], triple[1], triple[2]];
                t.sort();
                triples.insert(t);
            }
        }
    }

    triples
}

fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}
