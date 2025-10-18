use std::path::{Path, PathBuf};

pub fn get_path(name: &str) -> Option<PathBuf> {
    let file_path = Path::new(name);
    if file_path.is_absolute() {
        return Some(file_path.to_path_buf());
    }

    let cwd = std::env::current_dir().ok()?;
    Some(cwd.as_path().join(name))
}
