use std::path::PathBuf;

use commands::exec::language::Language;
use duct::{ cmd, Expression };

#[derive(Debug)]
pub struct Vb;

impl Vb {
    fn get_compiler(&self) -> String {
        if cfg!(windows) {
            "vbc".into()
        } else {
            "vbnc".into()
        }
    }
}

impl Language for Vb {
    fn get_lang_name(&self) -> String {
        "VB.Net".into()
    }

    fn get_source_file_ext(&self) -> String {
        ".vb".into()
    }

    fn pre_process_code(&self, code: &str, _src_path: &PathBuf) -> Option<String> {
        use regex::Regex;

        let re = Regex::new(r"Module\s*.*\s*Sub\s*Main\s*\(\s*\)").unwrap();
        if !re.is_match(&code) {
            let result = format!(r"
Imports System

Module Program
    Sub Main()
        {}
    End Sub
End Module", code);
            return Some(result);
        }

        None
    }

    fn get_compiler_command(&self, src_path: &PathBuf, exe_path: &PathBuf) -> Option<Expression> {
        let compiler = self.get_compiler();
        let out = format!("/out:{}", exe_path.to_str().unwrap());
        Some(cmd!(compiler, out, "/target:winexe", "/nologo", "/quiet", src_path))
    }

    fn get_execution_command(&self, path: &PathBuf) -> Expression {
        if cfg!(windows) {
            cmd!(path)
        } else {
            cmd!("mono", path)
        }
    }

    fn check_compiler_or_interpreter(&self) -> Expression {
        if cfg!(windows) {
            cmd!(self.get_compiler(), "/version")
        } else {
            cmd!(self.get_compiler(), "/nologo", "/?")
        }
    }
}
