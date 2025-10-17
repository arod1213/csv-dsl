use std::collections::HashMap;

pub enum DateType {
    YMD,
}

struct DateInfo {
    year: Option<String>,
    month: Option<String>,
    day: Option<String>,
}

fn find_separator(date_field: &str) -> Option<char> {
    let mut seps: HashMap<char, usize> = HashMap::new();
    for c in date_field.chars() {
        if c.is_digit(10) || c.is_alphabetic() || c == '\n' || c == '\r' {
            continue;
        }
        if let Some(count) = seps.get_mut(&c) {
            *count += 1;
        } else {
            seps.insert(c, 0);
        }
    }
    if seps.len() == 0 {
        return None;
    }
    match seps.iter().max_by(|(_, a), (_, b)| a.cmp(b)) {
        Some((c, _)) => return Some(c.clone()),
        _ => return None,
    };
}

enum DateError {
    UnsupportedFormat,
}
fn collect_date(field: &str) -> Result<[Option<String>; 3], DateError> {
    let Some(sep) = find_separator(field) else {
        return Err(DateError::UnsupportedFormat)
    };

    let parts: Vec<String> = field.split(sep).map(|s| s.to_string()).collect();
    if parts.len() < 3 {
        return Err(DateError::UnsupportedFormat);
    }

    // TODO: remove unnecessary cloning here
    let res = [
        Some(parts[0].clone()),
        Some(parts[1].clone()),
        Some(parts[2].clone()),
    ];
    Ok(res)
}

pub fn parse_date(field: &str) -> Result<DateType, DateError> {
    let fields = collect_date(field)?;

    todo!();
}
