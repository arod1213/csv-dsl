use crate::utils::clean_line;

pub fn collect_fields(line: &str, sep: &char) -> Vec<String> {
    let mut start: usize = 0;
    let mut fields: Vec<String> = vec![];
    let mut is_escaped: bool = false;

    for (i, c) in line.char_indices() {
        if c == '"' {
            is_escaped = !is_escaped;
        } else if !is_escaped && c == *sep {
            let new_line = clean_line(&line[start..i]);
            fields.push(new_line.to_string());
            start = i + sep.len_utf8(); // move past separator
        }
    }

    if start <= line.len() {
        let new_line = clean_line(&line[start..]);
        fields.push(new_line.to_string());
    }

    fields
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collect() {
        let ans = vec!["aidan,inc", "other", "0.12", "00"];
        let input = "\"aidan,inc\",other,0.12,00";
        let sep = ',';
        assert_eq!(collect_fields(input, &sep), ans);
    }
}
