pub fn spell(n: u64) -> String {
    if n == 0 {
        return "zero".to_string();
    }

    spell_number(n).trim().to_string()
}

fn spell_number(n: u64) -> String {
    let under_20 = [
        "", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        "ten", "eleven", "twelve", "thirteen", "fourteen", "fifteen",
        "sixteen", "seventeen", "eighteen", "nineteen",
    ];

    let tens = [
        "", "", "twenty", "thirty", "forty", "fifty",
        "sixty", "seventy", "eighty", "ninety",
    ];

    match n {
        1_000_000 => "one million".to_string(),
        1_000..=999_999 => {
            let thousands = n / 1_000;
            let rest = n % 1_000;
            let mut result = format!("{} thousand", spell_number(thousands));
            if rest > 0 {
                result.push(' ');
                result.push_str(&spell_number(rest));
            }
            result
        }
        100..=999 => {
            let hundreds = n / 100;
            let rest = n % 100;
            let mut result = format!("{} hundred", spell_number(hundreds));
            if rest > 0 {
                result.push(' ');
                result.push_str(&spell_number(rest));
            }
            result
        }
        20..=99 => {
            let t = n / 10;
            let r = n % 10;
            if r > 0 {
                format!("{}-{}", tens[t as usize], under_20[r as usize])
            } else {
                tens[t as usize].to_string()
            }
        }
        1..=19 => under_20[n as usize].to_string(),
        _ => "".to_string(), 
    }
}



