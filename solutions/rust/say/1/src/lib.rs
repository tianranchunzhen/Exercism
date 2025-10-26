const UNDER_20: &[&str] = &[
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

const TENS: &[&str] = &[
    "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
];

const SCALES: &[&str] = &[
    "thousand",
    "million",
    "billion",
    "trillion",
    "quadrillion",
    "quintillion",
];

fn encode_under_100(n: u64) -> Option<String> {
    if n == 0 {
        return None;
    }
    if n < 20 {
        return Some(UNDER_20[n as usize].to_string());
    }
    if n < 100 {
        let mut s = TENS[n as usize / 10].to_string();
        if n % 10 != 0 {
            s.push('-');
            s.push_str(UNDER_20[n as usize % 10]);
        }
        return Some(s);
    }
    None
}

fn encode_under_1000(n: u64) -> Option<String> {
    if n == 0 {
        return None;
    }
    if n < 100 {
        return encode_under_100(n);
    }

    let mut s = String::new();
    s.push_str(UNDER_20[n as usize / 100]);
    s.push_str(" hundred");
    if let Some(rem) = encode_under_100(n % 100) {
        s.push(' ');
        s.push_str(&rem);
    }
    Some(s)
}

pub fn encode(n: u64) -> String {
    if n == 0 {
        return "zero".to_string();
    }

    let mut parts = Vec::new();
    let mut rem = n;
    let mut scale_idx = 0;

    while rem > 0 {
        let chunk = rem % 1000;
        if let Some(encoded_chunk) = encode_under_1000(chunk) {
            let mut part = encoded_chunk;
            if scale_idx > 0 {
                part.push(' ');
                part.push_str(SCALES[scale_idx - 1]);
            }
            parts.push(part);
        }
        rem /= 1000;
        scale_idx += 1;
    }

    parts.iter().rev().cloned().collect::<Vec<_>>().join(" ")
}
