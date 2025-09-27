use std::{env, path::PathBuf};

pub fn path() -> Vec<PathBuf> {
    let path = env::var("PATH").unwrap();
    let mut result = Vec::new();

    for element in path.split(":") {
        let element = PathBuf::from(element);
        if element.exists() && element.is_dir() {
            result.push(element);
        }
    }

    return result;
}
