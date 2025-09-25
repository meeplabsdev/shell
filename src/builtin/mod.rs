// This file is GENERATED (build.rs)
        
mod r#cd;
mod r#false;
mod r#pwd;

use crate::shell::Shell;
use std::collections::HashMap;

#[allow(type_alias_bounds)]
pub type Sig = fn(&mut Shell, Vec<String>) -> i32;
pub fn builtins() -> HashMap<String, Sig> {
    let mut m: HashMap<String, Sig> = HashMap::new();
    m.insert("cd".to_string(), r#cd::function);
    m.insert("false".to_string(), r#false::function);
    m.insert("pwd".to_string(), r#pwd::function);
    return m;
}
