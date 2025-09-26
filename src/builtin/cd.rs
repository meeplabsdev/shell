use getopts::{Matches, Options};

use crate::{error::Error, shell::Shell};
use std::{
    env::{self, current_dir, home_dir, set_current_dir},
    path::{Component, PathBuf},
};

pub fn function(shell: &mut Shell, arguments: Vec<String>) -> i32 {
    let mut opts = Options::new();
    opts.optflag("p", "physical", "");
    let options = match opts.parse(arguments.clone()) {
        Ok(m) => m,
        Err(_) => {
            return -1;
        }
    };

    let path = get_path(options);
    if let Ok(path) = path {
        let oldpath = current_dir().unwrap();
        let result = set_current_dir(&path);
        if result.is_ok() {
            unsafe {
                env::set_var("PWD", path.canonicalize().unwrap());
                env::set_var("OLDPWD", oldpath.canonicalize().unwrap());
            }
        }

        return result.is_err() as i32;
    } else {
        let _ = shell.errln(path.unwrap_err().to_string());
    }

    return 1;
}

fn get_path(options: Matches) -> Result<PathBuf, Error> {
    let parts = options.free.clone();
    if parts.len() == 0 {
        return home_dir().ok_or("No home directory located".into());
    }

    if parts.len() == 1 && parts.get(0).unwrap() == "^" {
        if let Ok(oldpwd) = env::var("OLDPWD") {
            return Ok(PathBuf::from(oldpwd));
        } else {
            return Ok(PathBuf::from(".."));
        }
    }

    let default = PathBuf::from(parts.join(" "));
    let zeroth = parts.get(0).unwrap();
    if zeroth.starts_with(['/', '\\']) {
        return normalize(options, default);
    } else if zeroth.starts_with(".") || zeroth.starts_with("..") {
        return normalize(options, default);
    }

    if let Ok(cdpaths) = env::var("CDPATH") {
        let cdpaths: Vec<&str> = cdpaths.split(":").filter(|p| !p.eq(&"")).collect();
        for cdpath in cdpaths {
            let mut path = PathBuf::from(cdpath);
            path.push(zeroth);

            if path.is_dir() {
                return normalize(options, path);
            }

            let mut dotpath = PathBuf::from(cdpath);
            dotpath.push(".");
            dotpath.push(zeroth);

            if dotpath.is_dir() {
                return normalize(options, dotpath);
            }
        }
    }

    return normalize(options, default);
}

fn normalize(options: Matches, curpath: PathBuf) -> Result<PathBuf, Error> {
    let mut curpath = curpath;

    if !options.opt_present("p") {
        if let Some(scpath) = curpath.to_str()
            && scpath.starts_with(['/', '\\'])
        {
            curpath = PathBuf::from(env::var("PWD").unwrap()).join(curpath);
        }

        let mut components = curpath.components();
        let first = components.next().unwrap();

        let mut prev_path = PathBuf::from(current_dir()?).join(first);
        let mut prev_component = first.clone();
        for component in components {
            match component {
                Component::ParentDir => match prev_component {
                    Component::RootDir => {}
                    Component::ParentDir => {}
                    _ => {
                        if !prev_path.is_dir() {
                            return Err("failed enumerating path".into());
                        } else {
                            prev_path.pop();
                            continue;
                        }
                    }
                },
                _ => {}
            }

            prev_component = component;
            prev_path.push(component);
        }

        curpath = prev_path;
    }

    return match curpath.canonicalize() {
        Ok(curpath) => Ok(curpath),
        Err(err) => Err(err.into()),
    };
}
