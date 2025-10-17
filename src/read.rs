use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
};
pub fn read_line(reader: &mut BufReader<File>) -> Option<String> {
    let mut buf: String = "".to_string();
    let byte_count = match reader.read_line(&mut buf) {
        Ok(x) if x == 0 => return None,
        Ok(x) => x,
        Err(_) => return None,
    };
    Some(buf[0..byte_count].to_string())
}

pub fn get_path(name: &str) -> Option<PathBuf> {
    let file_path = Path::new(name);
    if file_path.is_absolute() {
        return Some(file_path.to_path_buf());
    }

    let cwd = std::env::current_dir().ok()?;
    Some(cwd.as_path().join(name))
}
