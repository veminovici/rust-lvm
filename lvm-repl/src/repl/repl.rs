use std::io::Write;

use anyhow::Error;
use lvm_core::Instruction;
use lvm_machine::VM;
use lvm_parser::ParseString;
use rustyline::error::ReadlineError;

use crate::ReplBuilder;

pub struct Repl {
    pub(crate) name: String,
    pub(crate) version: String,
    pub(crate) prompt: String,
    pub(crate) out: Box<dyn Write>,
    pub(crate) editor: rustyline::Editor<()>,
    pub(crate) vm: VM,
}

enum IterationResult {
    Continue,
    Break,
}

impl Repl {
    pub fn builder() -> ReplBuilder {
        ReplBuilder::new()
    }

    fn parse_instruction(input: &str) -> anyhow::Result<Instruction> {
        Instruction::parse_str(input)
            .map(|(_, i)| i)
            .map_err(|_e| Error::msg("Not an instruction"))
    }

    fn iterate(&mut self) -> anyhow::Result<IterationResult> {
        let readline = self.editor.readline(&self.prompt);
        match readline {
            Ok(line) => match line.as_str() {
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
                ":i" => {
                    writeln!(&mut self.out, "{}", self.vm)?;
                    Ok(IterationResult::Continue)
                }
                line => {
                    let instruction = Repl::parse_instruction(line)?;
                    match instruction {
                        Instruction::LoadI(load) => {
                            self.editor.add_history_entry(line);
                            writeln!(&mut self.out, "Executing: {}", &load)?;
                            self.vm.run_load(load)
                        }
                        Instruction::AddI(add) => {
                            self.editor.add_history_entry(line);
                            writeln!(&mut self.out, "Executing: {}", &add)?;
                            self.vm.run_add(add)
                        }
                    }
                    Ok(IterationResult::Continue)
                }
            },
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
        writeln!(
            &mut self.out,
            "Welcome to `{} - {}` repl!",
            self.name, self.version
        )?;

        while let IterationResult::Continue = self.iterate()? {}

        Ok(())
    }
}
