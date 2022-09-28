use std::io::Write;

use rustyline::error::ReadlineError;

use crate::ReplBuilder;

pub struct Repl {
    pub(crate) name: String,
    pub(crate) version: String,
    pub(crate) prompt: String,
    pub(crate) out: Box<dyn Write>,
    pub(crate) editor: rustyline::Editor::<()>,

}

enum IterationResult {
    Continue,
    Break,
}

impl Repl {
    pub fn builder() -> ReplBuilder {
        ReplBuilder::new()
    }

    fn iterate(&mut self) -> anyhow::Result<IterationResult> {

        let readline = self.editor.readline(&self.prompt);
        match readline {
            Ok(line) => {
                match line.as_str() {
                    ":q" => {
                        writeln!(&mut self.out, "Quiting")?;
                        Ok(IterationResult::Break)
                    }
                    ":h" => {
                        writeln!(&mut self.out, "{} - {} repl", self.name, self.version)?;
                        writeln!(&mut self.out, "Help here")?;
                        writeln!(&mut self.out, "  :h - prints the help")?;
                        writeln!(&mut self.out, "  :q - terminates the application")?;
                        writeln!(&mut self.out, "  :i - prints the internal information")?;
                        Ok(IterationResult::Continue)
                    }
                    _ => {
                        self.editor.add_history_entry(line.as_str());
                        writeln!(&mut self.out, "Line: {}", line)?;
                        Ok(IterationResult::Continue)
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                writeln!(&mut self.out, "CTRL-C")?;
                Ok(IterationResult::Break)
            }
            Err(err) => {
                writeln!(&mut self.out, "Error: {:?}", err)?;
                Ok(IterationResult::Break)
            }
        }
    }

    pub fn run(&mut self) -> anyhow::Result<()> {
        writeln!(&mut self.out, "Welcome to `{} - {}` repl!", self.name, self.version)?;

        while let IterationResult::Continue = self.iterate()? {}

        Ok(())
    }
}
