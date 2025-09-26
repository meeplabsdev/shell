use getopts::{Matches, Options};

use crate::{error::Error, shell::Shell};
use std::{
    env::{self, current_dir},
    path::PathBuf,
};

pub fn function(shell: &mut Shell, arguments: Vec<String>) -> i32 {
    let mut opts = Options::new();
    opts.optflag("p", "", "");
    let options = match opts.parse(arguments.clone()) {
        Ok(m) => m,
        Err(_) => {
            return -1;
        }
    };

    let path = get_path(options);
    if let Ok(path) = path {
        let _ = shell.writeln(path.canonicalize().unwrap().to_str().unwrap());

        return 0;
    } else {
        let _ = shell.errln(path.unwrap_err().to_string());
    }

    return 1;
}

fn get_path(options: Matches) -> Result<PathBuf, Error> {
    let mut curpath = current_dir().unwrap();

    if !options.opt_present("p") {
        curpath = PathBuf::from(env::var("PWD").unwrap());
    }

    return match curpath.canonicalize() {
        Ok(curpath) => Ok(curpath),
        Err(err) => Err(err.into()),
    };
}
