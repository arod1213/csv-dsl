pub fn clean_line(line: &str) -> String {
    let cleaned: String = line
        .chars()
        .filter(|&c| (c.is_ascii()) && c != '\n' && c != '\r' && c != '\\' && c != '"')
        .collect();
    cleaned.trim().to_owned()
}
