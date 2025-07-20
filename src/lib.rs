use std::collections::HashMap;

pub trait CompileServiceTrait {
    fn paths(&self) -> Vec<String>;
    fn load_compiler_from_path(&mut self, p: &str) -> global::Result<()>;
    fn add_file(&mut self, p: &str) -> global::Result<()>;
    fn compile(&mut self, get_output: fn(&str) -> global::Result<String>) -> global::Result<()>;
}
