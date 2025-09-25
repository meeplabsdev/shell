use std::{fs::File, io::Write, path::Path};

fn main() -> Result<(), std::io::Error> {
    let mut builtins: Vec<String> = Vec::new();
    for file in Path::new("src/builtin").read_dir()? {
        if let Ok(file) = file
            && let Ok(fname) = file.file_name().into_string()
            && file.metadata().unwrap().is_file()
            && fname.ends_with(".rs")
            && fname != "mod.rs"
            && fname != ".rs"
        {
            builtins.push(fname);
        }
    }

    let mut imports = String::new();
    let mut inserts = String::new();
    for b in builtins {
        let b = b.strip_suffix(".rs").unwrap();
        imports.push_str(&format!("mod r#{};\n", b));
        inserts.push_str(&format!(
            "    m.insert(\"{b}\".to_string(), r#{b}::function);\n",
            b = b
        ));
    }

    let builtins = format!(
        "// This file is GENERATED (build.rs)
        
{}
use crate::shell::Shell;
use std::collections::HashMap;

#[allow(type_alias_bounds)]
pub type Sig = fn(&mut Shell, Vec<String>) -> i32;
pub fn builtins() -> HashMap<String, Sig> {{
    let mut m: HashMap<String, Sig> = HashMap::new();
{}    return m;
}}
",
        imports, inserts
    );

    let mut file = File::create("src/builtin/mod.rs")?;
    file.write(builtins.as_bytes())?;

    let mut tests: Vec<String> = Vec::new();
    for file in Path::new("src/tests").read_dir()? {
        if let Ok(file) = file
            && let Ok(fname) = file.file_name().into_string()
            && file.metadata().unwrap().is_file()
            && fname.ends_with(".rs")
            && fname != "mod.rs"
            && fname != ".rs"
        {
            tests.push(fname);
        }
    }

    let mut imports = String::new();
    for t in tests {
        let t = t.strip_suffix(".rs").unwrap();
        imports.push_str(&format!("pub mod r#{};\n", t));
    }

    let tests = format!(
        "// This file is GENERATED (build.rs)

use crate::shell::Shell;
use crate::stringbuffer::StringBuffer;

#[cfg(test)]
{}
#[allow(dead_code)]
pub struct BuiltinResult {{
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
}}

#[allow(dead_code)]
pub fn std_builtin<S: AsRef<str>>(builtin: S, arguments: Vec<String>) -> BuiltinResult {{
    let stdin = StringBuffer::new();
    let stdout = StringBuffer::new();
    let stderr = StringBuffer::new();

    let mut shell = Shell::new(stdin, stdout.clone(), stderr.clone());
    let command = shell.builtin(&builtin.as_ref().to_string()).unwrap();
    let exit_code = command(&mut shell, arguments);

    return BuiltinResult {{
        exit_code,
        stdout: stdout.into(),
        stderr: stderr.into(),
    }};
}}
",
        imports
    );

    let mut file = File::create("src/tests/mod.rs")?;
    file.write(tests.as_bytes())?;

    return Ok(());
}
