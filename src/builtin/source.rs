use crate::{environment, shell::Shell};
use getopts::Options;
use std::{env, fs::File, io::Read, path::PathBuf};

pub fn function(shell: &mut Shell, arguments: Vec<String>) -> i32 {
    let mut opts = Options::new();
    opts.optopt("p", "", "", "path");
    let options = match opts.parse(arguments.clone()) {
        Ok(m) => m,
        Err(_) => {
            return -1;
        }
    };

    let mut parts = options.free.clone();
    if parts.len() < 1 {
        shell.errln("no filename provided").ok();
        return -1;
    }

    let filename = parts.remove(0);
    let mut filepath = None;

    let mut searchdirs = environment::path();
    searchdirs.push(env::current_dir().unwrap());

    if options.opt_present("p") {
        searchdirs = Vec::new();
        for element in options.opt_str("p").unwrap().split(":") {
            searchdirs.push(PathBuf::from(element));
        }
    }

    for path in searchdirs {
        let test = PathBuf::from(path).join(&filename);
        if test.is_file() {
            filepath = Some(test.to_path_buf());
            break;
        }
    }

    if filepath.is_none() {
        shell.errln("script not found").ok();
        return -1;
    }

    let filepath = filepath.unwrap();
    if !filepath.exists() || !filepath.is_file() {
        shell.errln("script not found").ok();
        return -1;
    }

    let mut content = String::new();
    {
        let mut file = File::open(filepath).unwrap();
        file.read_to_string(&mut content).ok();
    }

    for line in content.lines() {
        if shell.exec(line) != 0 {
            break;
        }
    }

    return 0;
}
