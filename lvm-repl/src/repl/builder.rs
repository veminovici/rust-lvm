use lvm_machine::VM;
use rustyline::{config::Configurer, ColorMode};

use crate::Repl;
use std::io::Write;

pub struct ReplBuilder {
    name: String,
    version: String,
    prompt: String,
    out: Box<dyn Write>,
    color_mode: ColorMode,
}

impl ReplBuilder {
    const DEFAULT_NAME: &str = "lvm repl";
    const DEFAULT_PROMPT: &str = ">";
    const VERSION: &str = "0.1.0";

    pub(crate) fn new() -> Self {
        ReplBuilder {
            name: ReplBuilder::DEFAULT_NAME.into(),
            prompt: ReplBuilder::DEFAULT_PROMPT.into(),
            version: ReplBuilder::VERSION.into(),
            out: Box::new(std::io::stderr()),
            color_mode: ColorMode::Disabled,
        }
    }

    pub fn build(self) -> Repl {
        let mut editor = rustyline::Editor::<()>::new().unwrap();
        editor.set_color_mode(self.color_mode);

        Repl {
            name: self.name,
            version: self.version,
            prompt: format!("{} ", self.prompt),
            out: self.out,
            editor,
            vm: VM::new(),
        }
    }

    pub fn with_name(mut self, n: String) -> Self {
        self.name = n;
        self
    }

    pub fn with_prompt(mut self, p: String) -> Self {
        self.prompt = p;
        self
    }

    pub fn with_color_mode(mut self) -> Self {
        self.color_mode = ColorMode::Enabled;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build() {
        let repl = Repl::builder()
            .with_name("Test".into())
            .with_prompt("*>".into())
            .build();
        assert_eq!("Test", repl.name);
        assert_eq!("*> ", repl.prompt);
    }
}
