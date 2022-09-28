mod repl;

pub use repl::*;

fn main() {
    let mut repl = Repl::builder()
        .with_name("Language VM".into())
        // .with_prompt("*>".into())
        .build();

    repl.run().unwrap();
}
