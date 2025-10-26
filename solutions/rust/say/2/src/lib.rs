const ONES: &[&str; 20] = &[
    "zero",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen",
];

const TENS: &[&str; 10] = &[
    "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
];

const SCALES: &[(u64, &str)] = &[
    (1_000_000_000_000_000_000, "quintillion"),
    (1_000_000_000_000_000, "quadrillion"),
    (1_000_000_000_000, "trillion"),
    (1_000_000_000, "billion"),
    (1_000_000, "million"),
    (1_000, "thousand"),
];

fn format_num(num: u64, div: u64, order: &str) -> String {
    let upper = num / div;
    let lower = num % div;
    match lower {
        0 => format!("{} {}", encode(upper), order),
        _ => format!("{} {} {}", encode(upper), order, encode(lower)),
    }
}

pub fn encode(num: u64) -> String {
    match num {
        0..=19 => ONES[num as usize].to_string(),
        20..=99 => {
            let upper = (num / 10) as usize;
            match num % 10 {
                0 => TENS[upper].to_string(),
                lower => format!("{}-{}", TENS[upper], encode(lower)),
            }
        }
        100..=999 => format_num(num, 100, "hundred"),
        _ => {
            // Find the largest scale that fits the number.
            let (div, order) = SCALES.iter().find(|&&(d, _)| num >= d).unwrap();
            format_num(num, *div, order)
        }
    }
}
